use geo::{map_coords::MapCoordsInplace, LineString, Point, Polygon};
use geographiclib_rs::{DirectGeodesic, Geodesic};
use geojson::GeoJson;
use std::convert::TryInto;
use std::f64::consts::PI;
use std::{default::Default, iter::FromIterator};

use structopt::StructOpt;

/// Generates a clockboard centered around a point. Returns a GeoJSON object with one feature per
/// zone.
pub fn clockboard(
    center: Point<f64>,
    params: Params,
    // TODO Clip to a boundary
    //boundary: Option<Polygon<f64>>,
) -> GeoJson {
    let polygons: Vec<Polygon<f64>> = if params.num_segments == 1 {
        params
            .distances
            .iter()
            .map(|distance| {
                make_circle(
                    center,
                    *distance,
                    params.num_vertices_arc * params.num_segments,
                    params.projected,
                )
            })
            .collect()
    } else {
        // For each circle radius
        params
            .distances
            .iter()
            .enumerate()
            .flat_map(|(idx, _)| {
                let irad = params.distances[idx];
                let irad_inner = if idx == 0 {
                    0.0
                } else {
                    params.distances[(idx - 1)]
                };
                let num_segs = if idx == 0 { 1 } else { params.num_segments };
                (0..num_segs)
                    .map(|jdx| {
                        if idx != 0 {
                            clock_polygon(
                                center,
                                irad,
                                irad_inner,
                                params.num_vertices_arc,
                                params.num_segments,
                                jdx,
                                params.projected,
                            )
                        } else {
                            make_circle(
                                center,
                                irad,
                                params.num_vertices_arc * params.num_segments,
                                params.projected,
                            )
                        }
                    })
                    .collect::<Vec<Polygon<f64>>>()
            })
            .map(|mut poly| {
                round(&mut poly, params.precision);
                poly
            })
            .collect()
    };

    let gc = geo::GeometryCollection::from_iter(polygons);
    let fc = geojson::FeatureCollection::from(&gc);
    GeoJson::from(fc)
}

/// Configures a clockboard diagram
#[derive(StructOpt, Debug)]
#[structopt(name = "zb")]
pub struct Params {
    /// The number of radial segments. Defaults to 12, like the hours on a clock.
    #[structopt(short = "s", long, default_value = "12")]
    num_segments: usize,

    /// The distances between concentric rings. `triangular_sequence` is useful to generate these
    /// distances.
    #[structopt(
        short,
        long,
        default_value = "1.0,3.0,6.0,10.0,15.0",
        use_delimiter = true
    )]
    distances: Vec<f64>,

    /// The number of vertices per arc. Higher values approximate a circle more accurately.
    #[structopt(short = "v", long, default_value = "10")]
    num_vertices_arc: usize,

    /// The number of decimal places in the resulting output GeoJSON files.
    /// Set to 6 by default. Larger numbers mean more precision, but larger file sizes.
    #[structopt(short, long, default_value = "6")]
    precision: usize,

    /// Is the data projected?
    #[structopt(long)]
    projected: bool,
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

fn arc_points(
    num_circles: usize,
    idx: usize,
    angular_offset: f64,
    center: Point<f64>,
    radius: f64,
) -> Point<f64> {
    let angle: f64 = 2.0 * PI / (num_circles as f64) * (idx as f64) + angular_offset;
    let x = center.x() + radius * angle.sin();
    let y = center.y() + radius * angle.cos();
    Point::new(x, y)
}

fn arc_points_geodesic(
    crs: &Geodesic,
    num_circles: usize,
    idx: usize,
    angular_offset: f64,
    center: Point<f64>,
    radius: f64,
) -> Point<f64> {
    let angle: f64 = 360.0 / (num_circles as f64) * (idx as f64) + angular_offset;
    let (y, x) = crs.direct(center.y(), center.x(), angle, radius * 1000.0);
    Point::new(x, y)
}

fn round(poly: &mut Polygon<f64>, precision: usize) {
    // Convert precision (e.g. 5) into power of 10 (e.g. 10^5):
    let p = 10_usize.pow(precision.try_into().unwrap()) as f64;
    poly.map_coords_inplace(|&(x, y)| (f64::trunc(x * p) / p, f64::trunc(y * p) / p))
}

fn make_circle(
    center: Point<f64>,
    radius: f64,
    num_vertices: usize,
    projected: bool,
) -> Polygon<f64> {
    let circle_points: Vec<Point<f64>> = if projected {
        (0..num_vertices)
            .map(|idx| {
                let angle: f64 = 2.0 * PI / (num_vertices as f64) * (idx as f64);
                let x = center.x() + radius * angle.cos();
                let y = center.y() + radius * angle.sin();
                Point::new(x, y)
            })
            .collect()
    } else {
        let crs = Geodesic::wgs84();
        (0..num_vertices)
            .map(|idx| {
                let angle: f64 = 360.0 / (num_vertices as f64) * (idx as f64);
                let (y, x) = crs.direct(center.y(), center.x(), angle, radius * 1000.0);
                Point::new(x, y)
            })
            .collect()
    };
    Polygon::new(LineString::from(circle_points), vec![])
}

fn clock_polygon(
    center: Point<f64>,
    radius_outer: f64,
    radius_inner: f64,
    num_vertices_arc: usize,
    num_segments: usize,
    seg: usize,
    projected: bool,
) -> Polygon<f64> {
    let num_vertices_circle = num_vertices_arc * num_segments;
    let idx1 = seg * num_vertices_arc;
    let idx2 = 1 + (seg + 1) * num_vertices_arc;
    // Angle offset so the first segment is North
    let angular_offset = std::f64::consts::PI / (num_segments as f64);
    let arcs: Vec<Point<f64>> = if projected {
        (idx1..idx2)
            .map(|idx| {
                arc_points(
                    num_vertices_circle,
                    idx,
                    angular_offset,
                    center,
                    radius_outer,
                )
            })
            .chain((idx1..idx2).rev().map(|idx| {
                arc_points(
                    num_vertices_circle,
                    idx,
                    angular_offset,
                    center,
                    radius_inner,
                )
            }))
            .collect()
    } else {
        let crs = Geodesic::wgs84();
        (idx1..idx2)
            .map(|idx| {
                arc_points_geodesic(
                    &crs,
                    num_vertices_circle,
                    idx,
                    angular_offset,
                    center,
                    radius_outer,
                )
            })
            .chain((idx1..idx2).rev().map(|idx| {
                arc_points_geodesic(
                    &crs,
                    num_vertices_circle,
                    idx,
                    angular_offset,
                    center,
                    radius_inner,
                )
            }))
            .collect()
    };
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
