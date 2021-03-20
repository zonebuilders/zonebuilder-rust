use geo::{LineString, Point, Polygon};
use std::default::Default;

// See https://stackoverflow.com/questions/24047686
#[derive(Debug)]
pub struct Params {
    n_circles: usize,
    num_segments: usize,
    distances: Vec<f64>,
}

// https://doc.rust-lang.org/std/default/trait.Default.html

impl Default for Params {
    fn default() -> Self {
        // todo: distances should be:
        // zonebuilder::zb_100_triangular_numbers
        // 1    3    6   10   15   21   28   36   45   55   66 ...
        Params { n_circles: 5, num_segments: 12, distances: vec![1.0, 3.0, 6.0, 10.0, 15.0]}
    }
}


pub fn clockboard(
    centerpoint: Point<f64>,
    num_segments: usize,
    radii: Vec<f64>,
    boundary: Option<Polygon<f64>>,
    params: Params,
) -> Vec<Polygon<f64>> {
    // test options worked // no idea if this next line is useful (probably not)
    // let options = Params {n_circles: 42 ..Default::Default} 
    println!("{:?}", n_circles);
    let mut polygons = Vec::new();
    let circle = makecircle(centerpoint, radii[0]);
    polygons.push(circle);
    polygons
}

fn makecircle(centerpoint: Point<f64>, radius: f64) -> Polygon<f64> {
    // hardcode num vertices - can be argument later
    let num_vertices = 121;

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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let polygon_list = clockboard(Point::new(0.0, 0.0), 2, vec![1.0], None);
        let geojson_list = geojson::Value::from(&polygon_list[0]);
        println!("{}", geojson_list);
        assert!(false)
    }
}
