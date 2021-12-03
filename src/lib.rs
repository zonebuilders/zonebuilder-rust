use geo::{map_coords::MapCoordsInplace, LineString, Point, Polygon};
use geographiclib_rs::{DirectGeodesic, Geodesic};
use geojson::GeoJson;
use std::convert::TryInto;
use std::default::Default;
use std::f64::consts::PI;
use structopt::StructOpt;

/// Generates a clockboard centered around a point. Returns a GeoJSON object with one feature per
/// zone, including a `label` property.
pub fn clockboard(
    center: Point<f64>,
    params: Params,
    // TODO Clip to a boundary
    //boundary: Option<Polygon<f64>>,
) -> GeoJson {
    // Each zone has a label and a polygon
    let mut zones: Vec<(String, Polygon<f64>)> = Vec::new();

    let crs = if params.projected {
        None
    } else {
        Some(Geodesic::wgs84())
    };

    // The innermost zone is just a circle
    zones.push((
        "A".to_string(),
        Polygon::new(
            make_circle(
                center,
                params.distances[0],
                params.num_vertices_arc * params.num_segments,
                crs,
            ),
            Vec::new(),
        ),
    ));

    let mut ring_label = 'B';
    for pair in params.distances.windows(2) {
        let (inner_radius, outer_radius) = (pair[0], pair[1]);

        if params.num_segments == 1 {
            // Clip out the inner hole
            let outer_ring = make_circle(
                center,
                outer_radius,
                params.num_vertices_arc * params.num_segments,
                crs,
            );
            let inner_ring = make_circle(
                center,
                inner_radius,
                params.num_vertices_arc * params.num_segments,
                crs,
            );
            zones.push((
                ring_label.to_string(),
                Polygon::new(outer_ring, vec![inner_ring]),
            ));
        } else {
            // Each ring is chopped into num_segments
            for idx in 0..params.num_segments {
                zones.push((
                    format!("{}{:02}", ring_label, idx + 1),
                    clock_polygon(
                        center,
                        outer_radius,
                        inner_radius,
                        params.num_vertices_arc,
                        params.num_segments,
                        idx,
                        crs,
                    ),
                ));
            }
        }

        // B -> C, C -> D, etc in ASCII
        ring_label = std::char::from_u32(ring_label as u32 + 1).expect("too may rings");
    }

    for (_, poly) in &mut zones {
        round(poly, params.precision);
    }

    // Transform the labelled polygons from the geo crate into the geojson crate. Ideally we could
    // directly map each (label, polygon) into a geojson::Feature, but the conversion APIs are
    // confusing...
    let geom_collection: geo::GeometryCollection<f64> =
        zones.iter().map(|(_, poly)| poly.clone()).collect();
    let mut feature_collection = geojson::FeatureCollection::from(&geom_collection);
    for (feature, (label, _)) in feature_collection.features.iter_mut().zip(zones) {
        let mut properties = serde_json::Map::new();
        properties.insert("label".to_string(), label.into());
        feature.properties = Some(properties);
    }
    GeoJson::from(feature_collection)
}

/// Configures a clockboard diagram
#[derive(StructOpt, Debug)]
#[structopt(name = "zb")]
pub struct Params {
    /// The number of radial segments. Defaults to 12, like the hours on a clock.
    #[structopt(short = "s", long, default_value = "12")]
    pub num_segments: usize,

    /// The distances between concentric rings. `triangular_sequence` is useful to generate these
    /// distances.
    #[structopt(
        short,
        long,
        default_value = "1.0,3.0,6.0,10.0,15.0",
        use_delimiter = true
    )]
    pub distances: Vec<f64>,

    /// The number of vertices per arc. Higher values approximate a circle more accurately.
    #[structopt(short = "v", long, default_value = "10")]
    pub num_vertices_arc: usize,

    /// The number of decimal places in the resulting output GeoJSON files.
    /// Set to 6 by default. Larger numbers mean more precision, but larger file sizes.
    #[structopt(short, long, default_value = "6")]
    pub precision: usize,

    /// Is the data projected?
    #[structopt(long)]
    pub projected: bool,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            num_segments: 12,
            distances: triangular_sequence(5),
            num_vertices_arc: 10,
            precision: 6,
            projected: false,
        }
    }
}

/// Returns the first `n` [triangular numbers](https://en.wikipedia.org/wiki/Triangular_number),
/// excluding the 0th.
pub fn triangular_sequence(n: usize) -> Vec<f64> {
    (1..=n).map(|i| 0.5 * (i as f64) * (i + 1) as f64).collect()
}

fn arc_point(
    num_circles: usize,
    idx: usize,
    num_segments: usize,
    center: Point<f64>,
    radius: f64,
    crs: Option<Geodesic>,
) -> Point<f64> {
    if let Some(crs) = crs {
        let offset = 180.0 / (num_segments as f64);
        let angle: f64 = 360.0 / (num_circles as f64) * (idx as f64) + offset;
        let (y, x) = crs.direct(center.y(), center.x(), angle, radius * 1000.0);
        Point::new(x, y)
    } else {
        let offset = std::f64::consts::PI / (num_segments as f64);
        let angle: f64 = 2.0 * PI / (num_circles as f64) * (idx as f64) + offset;
        let x = center.x() + radius * angle.sin();
        let y = center.y() + radius * angle.cos();
        Point::new(x, y)
    }
}

fn round(poly: &mut Polygon<f64>, precision: usize) {
    // Convert precision (e.g. 5) into power of 10 (e.g. 10^5):
    let p = 10_usize.pow(precision.try_into().unwrap()) as f64;
    poly.map_coords_inplace(|&(x, y)| (f64::trunc(x * p) / p, f64::trunc(y * p) / p))
}

// Returns a circle as a closed LineString.
fn make_circle(
    center: Point<f64>,
    radius: f64,
    num_vertices: usize,
    crs: Option<Geodesic>,
) -> LineString<f64> {
    let circle_points: Vec<Point<f64>> = if let Some(crs) = crs {
        (0..num_vertices)
            .map(|idx| {
                let angle: f64 = 360.0 / (num_vertices as f64) * (idx as f64);
                let (y, x) = crs.direct(center.y(), center.x(), angle, radius * 1000.0);
                Point::new(x, y)
            })
            .collect()
    } else {
        (0..num_vertices)
            .map(|idx| {
                let angle: f64 = 2.0 * PI / (num_vertices as f64) * (idx as f64);
                let x = center.x() + radius * angle.cos();
                let y = center.y() + radius * angle.sin();
                Point::new(x, y)
            })
            .collect()
    };
    LineString::from(circle_points)
}

fn clock_polygon(
    center: Point<f64>,
    radius_outer: f64,
    radius_inner: f64,
    num_vertices_arc: usize,
    num_segments: usize,
    seg: usize,
    crs: Option<Geodesic>,
) -> Polygon<f64> {
    assert!(radius_outer > radius_inner);
    let num_vertices_circle = num_vertices_arc * num_segments;
    let idx1 = seg * num_vertices_arc;
    let idx2 = 1 + (seg + 1) * num_vertices_arc;
    let arcs: Vec<Point<f64>> = (idx1..idx2)
        .map(|idx| {
            arc_point(
                num_vertices_circle,
                idx,
                num_segments,
                center,
                radius_outer,
                crs,
            )
        })
        .chain((idx1..idx2).rev().map(|idx| {
            arc_point(
                num_vertices_circle,
                idx,
                num_segments,
                center,
                radius_inner,
                crs,
            )
        }))
        .collect();
    Polygon::new(LineString::from(arcs), vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_sequence() {
        assert!(triangular_sequence(0).is_empty());
        assert_eq!(vec![1.0], triangular_sequence(1));
        assert_eq!(vec![1.0, 3.0], triangular_sequence(2));
        assert_eq!(vec![1.0, 3.0, 6.0], triangular_sequence(3));
        assert_eq!(vec![1.0, 3.0, 6.0, 10.0], triangular_sequence(4));
    }

    #[test]
    fn test_number_of_zones() {
        let args: Vec<String> = Vec::new();
        let params = Params::from_iter(args);
        let gj = clockboard(Point::new(0.0, 0.0), params);
        if let GeoJson::FeatureCollection(fc) = gj {
            assert_eq!(49, fc.features.len());
        } else {
            panic!("not a feature collection");
        }
    }
}

// This code exposes make_clockboard as a regular Javascript function. It returns a GeoJSON string.
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn make_clockboard(lat: f64, lon: f64, distances: Vec<f64>, num_segments: usize) -> String {
    let args: Vec<String> = Vec::new();
    let mut params = Params::from_iter(args);
    params.distances = distances;
    params.num_segments = num_segments;
    let gj = clockboard(Point::new(lon, lat), params);
    serde_json::to_string(&gj).unwrap()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn generate_triangular_sequence(n: usize) -> Vec<f64> {
    triangular_sequence(n)
}
