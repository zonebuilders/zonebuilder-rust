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
    num_vertices: usize,
    precision: usize,
}

// https://doc.rust-lang.org/std/default/trait.Default.html

impl Default for Params {
    fn default() -> Self {
        // todo: distances should be:
        // zonebuilder::zb_100_triangular_numbers
        // 1    3    6   10   15   21   28   36   45   55   66 ...
        Params {
            n_circles: 5,
            num_segments: 12,
            distances: vec![1.0, 3.0, 6.0, 10.0, 15.0],
            // num_vertices: 121,
            num_vertices: 121,
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
    let mut irad_inner: f64 = 0.0;
    if params.num_segments == 1 {
        for i in params.distances {
            let circle = makecircle(centerpoint, i, params.num_vertices);
            polygons.push(circle);
        }
    } else {
        for i in 0..params.distances.len() {
            let irad = params.distances[i];
            if i == 0 {
                let irad_inner = 0.0;
            } else {
                let irad_inner = params.distances[(i - 1)];
            }
            for j in 0..params.num_segments {
                let circle = clockpoly(
                    centerpoint,
                    irad,
                    irad_inner,
                    params.num_vertices,
                    params.num_segments,
                    j,
                );
                polygons.push(circle);
            }
        }
    }

    for polygon in &mut polygons {
        round(polygon, params.precision);
    }

    let mut features: Vec<Feature> = polygons
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
pub fn clockpoly(
    centerpoint: Point<f64>,
    radius: f64,
    inner_radius: f64,
    num_vertices: usize,
    num_segments: usize,
    seg: usize,
) -> Polygon<f64> {
    let mut circle_points = Vec::new();
    let mut circle_points_inner = Vec::new();

    // Sequence of vertices
    // in R round(seq(1, 13, length.out = 12))
    // Number of vertices per segment
    let n = num_vertices / num_segments;
    let f = 0 + (seg) * n;
    let t = 0 + (seg + 1) * n;
    // Outer radius
    let a = f..t;
    for i in a {
        let angle: f64 = 2.0 * std::f64::consts::PI / (num_vertices as f64) * (i as f64);
        let x = centerpoint.x() + radius * angle.cos();
        let y = centerpoint.y() + radius * angle.sin();
        // println!("{}", x);

        circle_points.push(Point::new(x, y));
    }
    // Inner radius
    for i in (f..t).rev() {
        let angle: f64 = 2.0 * std::f64::consts::PI / (num_vertices as f64) * (i as f64);
        let x = centerpoint.x() + inner_radius * angle.cos();
        let y = centerpoint.y() + inner_radius * angle.sin();
        circle_points_inner.push(Point::new(x, y));
    }

    let circle_points_all = [circle_points, circle_points_inner].concat();
    let polygon = Polygon::new(LineString::from(circle_points_all), vec![]);
    polygon
}
