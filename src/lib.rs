use geo::{map_coords::MapCoordsInplace, LineString, Point, Polygon};
use geojson::{Feature, FeatureCollection, GeoJson, Geometry};
use std::convert::TryInto;
use std::default::Default;

// See https://stackoverflow.com/questions/24047686
#[derive(Debug)]
pub struct Params {
    n_circles: usize,
    num_segments: usize,
    distances: Vec<f64>,
    num_vertices_arc: usize,
    precision: usize,
}

// https://doc.rust-lang.org/std/default/trait.Default.html

impl Default for Params {
    fn default() -> Self {
        // default: triangular number sequence
        Params {
            n_circles: 5,
            num_segments: 12,
            distances: vec![1.0, 3.0, 6.0, 10.0, 15.0],
            num_vertices_arc: 10,
            precision: 6,
        }
    }
}

fn round(poly: &mut Polygon<f64>, precision: usize) {
    let p = 10_usize.pow(precision.try_into().unwrap()) as f64;
    poly.map_coords_inplace(|&(x, y)| (f64::trunc(x * p) / p, f64::trunc(y * p) / p))
}

pub fn clockboard(
    centerpoint: Point<f64>,
    params: Params,
    //boundary: Option<Polygon<f64>>,
) -> GeoJson {
    let mut polygons = Vec::new();
    let mut irad_inner: f64;
    if params.num_segments == 1 {
        for i in params.distances {
            let zone = makecircle(centerpoint, i, params.num_vertices_arc * params.num_segments);
            polygons.push(zone);
        }
    } else {
        for i in 0..params.distances.len() {
            let irad = params.distances[i];
            if i == 0 {
                irad_inner = 0.0;
            } else {
                irad_inner = params.distances[(i - 1)];
            }
            for j in 0..params.num_segments {
                let zone = clockpoly(
                    centerpoint,
                    irad,
                    irad_inner,
                    params.num_vertices_arc,
                    params.num_segments,
                    j,
                );
                polygons.push(zone);
            }
        }
    }

    for polygon in &mut polygons {
        round(polygon, params.precision);
    }

    let features: Vec<Feature> = polygons
        .iter()
        .map(|poly| Feature {
            bbox: None,
            geometry: Some(Geometry::from(poly)),
            id: None,
            properties: None,
            foreign_members: None,
        })
        .collect();

    let fc = FeatureCollection {
        bbox: None,
        features,
        foreign_members: None,
    };

    let gj = GeoJson::from(fc);
    gj
}

fn makecircle(centerpoint: Point<f64>, radius: f64, num_vertices: usize) -> Polygon<f64> {
    let mut circle_points = Vec::new();
    for i in 0..num_vertices {
        let angle: f64 = 2.0 * std::f64::consts::PI / (num_vertices as f64) * (i as f64);
        let x = centerpoint.x() + radius * angle.cos();
        let y = centerpoint.y() + radius * angle.sin();
        circle_points.push(Point::new(x, y));
    }
    let polygon = Polygon::new(LineString::from(circle_points), vec![]);
    polygon
}

// Make a single clock polygon
fn clockpoly(
    centerpoint: Point<f64>,
    radius_outer: f64,
    radius_inner: f64,
    num_vertices_arc: usize,
    num_segments: usize,
    seg: usize,
) -> Polygon<f64> {
    let mut arc_outer = Vec::new();
    let mut arc_inner = Vec::new();

    // Sequence of vertices
    // in R round(seq(from, to, length.out = num_segments))
    // Number of vertices per segment
    let n = num_vertices_arc * num_segments;
    let f = seg * n;
    let t = 1 + (seg + 1) * n;
    let seq = f..t;
    let seq_reverse = (f..t).rev();
    for i in seq {
        let angle: f64 = 2.0 * std::f64::consts::PI / (n as f64) * (i as f64);
        let x = centerpoint.x() + radius_outer * angle.cos();
        let y = centerpoint.y() + radius_outer * angle.sin();
        arc_outer.push(Point::new(x, y));
    }
    for i in seq_reverse {
        let angle: f64 = 2.0 * std::f64::consts::PI / (n as f64) * (i as f64);
        let x = centerpoint.x() + radius_inner * angle.cos();
        let y = centerpoint.y() + radius_inner * angle.sin();
        arc_inner.push(Point::new(x, y));
    }
    let arcs = [arc_outer, arc_inner].concat();
    let polygon = Polygon::new(LineString::from(arcs), vec![]);
    polygon
}

// Todo: get this working and use in clockpoly: refactor
// fn arc(angle1: f64, angle2: f64, num_vertices: usize, radius: f64, center: Point<f64>) -> Vec<Point<f64>> {
//     let mut arc = Vec::new();
//     // Todo: calculate sequence of numbers to iterate on
//     // let seq = ...
//     for i in seq {
//         // let angle = ...
//         let x = center.x() + radius * angle.cos();
//         let y = center.y() + radius * angle.sin();
//         arc.push(Point::new(x, y));
//     }

//     arc.push(x, y);
// }
