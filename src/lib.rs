use geo::{LineString, Point, Polygon, coords_iter::CoordsIter, map_coords::MapCoordsInplace};
use std::default::Default;

// See https://stackoverflow.com/questions/24047686
#[derive(Debug)]
pub struct Params {
    n_circles: usize,
    num_segments: usize,
    distances: Vec<f64>,
    num_vertices: usize,
    precision: usize
}

// https://doc.rust-lang.org/std/default/trait.Default.html

impl Default for Params {
    fn default() -> Self {
        // todo: distances should be:
        // zonebuilder::zb_100_triangular_numbers
        // 1    3    6   10   15   21   28   36   45   55   66 ...
        Params { n_circles: 5, num_segments: 12, distances: vec![1.0, 3.0, 6.0, 10.0, 15.0], num_vertices: 121, precision: 5}
    }
}

fn round(poly: &mut Polygon<f64>, precision: usize) {
    // hardcoded 2 d.p. todo: update
    poly.map_coords_inplace(|&(x, y)| (f64::trunc(x  * 100.0) / 100.0, f64::trunc(y  * 100.0) / 100.0))    
}

pub fn clockboard(
    centerpoint: Point<f64>,
    params: Params,
    boundary: Option<Polygon<f64>>,
) -> Vec<Polygon<f64>> {
    let mut polygons = Vec::new();
    let circle = makecircle(centerpoint, params.distances[0], params.num_vertices); 
    polygons.push(circle);
    polygons
}

fn makecircle(centerpoint: Point<f64>, radius: f64, num_vertices: usize) -> Polygon<f64> {
    let mut circle_points = Vec::new();
    // in R: 1:num_vertices
    for i in 0..num_vertices {
        let angle: f64 = 2.0 * std::f64::consts::PI / (num_vertices as f64) * (i as f64);
        let x = centerpoint.x() + radius * angle.cos();
        let y = centerpoint.y() + radius * angle.sin();
        circle_points.push(Point::new(x, y));
    }
    let polygon = Polygon::new(LineString::from(circle_points), vec![]);
    polygon
}
