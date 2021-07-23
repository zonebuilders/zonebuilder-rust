use geo::{map_coords::MapCoordsInplace, LineString, Point, Polygon};
use geographiclib_rs::{DirectGeodesic, Geodesic};
use geojson::GeoJson;
use std::convert::TryInto;
use std::f64::consts::PI;
use std::{default::Default, iter::FromIterator};

// use std::path::PathBuf;
use structopt::StructOpt;

/// Build zones
#[derive(StructOpt, Debug)]
#[structopt(name = "zb")]
pub struct Params {
    /// Set n_circles
    #[structopt(short = "c", long, default_value = "5")]
    n_circles: usize,

    /// Number of radial segments (12 by default)
    #[structopt(short = "s", long, default_value = "12")]
    num_segments: usize,

    /// Distances between concentric rings.
    /// first 5 values of the triangular number sequence
    /// by default, entered as -d 1.0,3.0,6.0,10.0,15.0
    #[structopt(
        short,
        long,
        default_value = "1.0,3.0,6.0,10.0,15.0",
        use_delimiter = true
    )]
    distances: Vec<f64>,

    /// Number of vertices per arc
    #[structopt(short = "v", long, default_value = "5")]
    num_vertices_arc: usize,

    /// Number of decimal places in the resulting output (GeoJSON) files.
    /// Set to 6 by default. Larger numbers mean more precision but
    /// larger file sizes.
    #[structopt(short, long, default_value = "6")]
    precision: usize,

    /// Is the data projected?
    /// False by default.
    #[structopt(long)]
    projected: bool,
    // /// Output file
    // #[structopt(short, long)]
    // output: PathBuf,
}

impl Default for Params {
    fn default() -> Self {
        // default: triangular number sequence
        Params {
            n_circles: 5,
            num_segments: 12,
            distances: vec![1.0, 3.0, 6.0, 10.0, 15.0],
            num_vertices_arc: 10,
            precision: 6,
            projected: false,
        }
    }
}

fn arcpoints(
    num_circles: usize,
    idx: usize,
    angular_offset: f64,
    centerpoint: Point<f64>,
    radius: f64,
) -> Point<f64> {
    let angle: f64 = 2.0 * PI / (num_circles as f64) * (idx as f64) + angular_offset;
    let x = centerpoint.x() + radius * angle.sin();
    let y = centerpoint.y() + radius * angle.cos();
    Point::new(x, y)
}

fn arcpoints_geodesic(
    crs: &Geodesic,
    num_circles: usize,
    idx: usize,
    angular_offset: f64,
    centerpoint: Point<f64>,
    radius: f64,
) -> Point<f64> {
    let angle: f64 = 360.0 / (num_circles as f64) * (idx as f64) + angular_offset;
    let (y, x) = crs.direct(centerpoint.y(), centerpoint.x(), angle, radius * 1000.0);
    Point::new(x, y)
}

fn round(poly: &mut Polygon<f64>, precision: usize) {
    // Convert precision (e.g. 5) into power of 10 (e.g. 10^5):
    let p = 10_usize.pow(precision.try_into().unwrap()) as f64;
    poly.map_coords_inplace(|&(x, y)| (f64::trunc(x * p) / p, f64::trunc(y * p) / p))
}

pub fn clockboard(
    centerpoint: Point<f64>,
    params: Params,
    // Todo: add boundary option
    //boundary: Option<Polygon<f64>>,
) -> GeoJson {
    let polygons: Vec<Polygon<f64>> = if params.num_segments == 1 {
        params
            .distances
            .iter()
            .map(|distance| {
                makecircle(
                    centerpoint,
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
                            clockpoly(
                                centerpoint,
                                irad,
                                irad_inner,
                                params.num_vertices_arc,
                                params.num_segments,
                                jdx,
                                params.projected,
                            )
                        } else {
                            makecircle(
                                centerpoint,
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

fn makecircle(
    centerpoint: Point<f64>,
    radius: f64,
    num_vertices: usize,
    projected: bool,
) -> Polygon<f64> {
    let circle_points: Vec<Point<f64>> = if projected {
        (0..num_vertices)
            .map(|idx| {
                let angle: f64 = 2.0 * PI / (num_vertices as f64) * (idx as f64);
                let x = centerpoint.x() + radius * angle.cos();
                let y = centerpoint.y() + radius * angle.sin();
                Point::new(x, y)
            })
            .collect()
    } else {
        let crs = Geodesic::wgs84();
        (0..num_vertices)
            .map(|idx| {
                let angle: f64 = 360.0 / (num_vertices as f64) * (idx as f64);
                let (y, x) = crs.direct(centerpoint.y(), centerpoint.x(), angle, radius * 1000.0);
                Point::new(x, y)
            })
            .collect()
    };
    Polygon::new(LineString::from(circle_points), vec![])
}

// Make a single clock polygon
fn clockpoly(
    centerpoint: Point<f64>,
    radius_outer: f64,
    radius_inner: f64,
    num_vertices_arc: usize,
    num_segments: usize,
    seg: usize,
    projected: bool,
) -> Polygon<f64> {
    // Sequence of vertices
    // in R round(seq(from, to, length.out = num_segments))
    // Number of vertices per segment
    let nv = num_vertices_arc;
    // Number of vertices in the circle
    let nc = num_vertices_arc * num_segments;
    let from_iterator = seg * nv;
    let to_iterator = 1 + (seg + 1) * nv;
    // Angle offset so first segment is North
    let angular_offset = std::f64::consts::PI / (num_segments as f64);
    let arcs: Vec<Point<f64>> = if projected {
        (from_iterator..to_iterator)
            .map(|idx| arcpoints(nc, idx, angular_offset, centerpoint, radius_outer))
            .chain(
                (from_iterator..to_iterator)
                    .rev()
                    .map(|idx| arcpoints(nc, idx, angular_offset, centerpoint, radius_inner)),
            )
            .collect()
    } else {
        let crs = Geodesic::wgs84();
        (from_iterator..to_iterator)
            .map(|idx| arcpoints_geodesic(&crs, nc, idx, angular_offset, centerpoint, radius_outer))
            .chain((from_iterator..to_iterator).rev().map(|idx| {
                arcpoints_geodesic(&crs, nc, idx, angular_offset, centerpoint, radius_inner)
            }))
            .collect()
    };
    Polygon::new(LineString::from(arcs), vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let args: Vec<String> = Vec::new();
        let params = Params::from_iter(args);
        eprintln!("{:?}", params); // print parameters
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
pub fn make_clockboard(lat: f64, lon: f64, num_circles: usize, num_segments: usize) -> String {
    let args: Vec<String> = Vec::new();
    let mut params = Params::from_iter(args);
    params.n_circles = num_circles;
    params.num_segments = num_segments;
    let gj = clockboard(Point::new(lon, lat), params);
    serde_json::to_string(&gj).unwrap()
}
