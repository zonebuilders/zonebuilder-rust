use geo::{LineString, Point, Polygon};

pub fn clockboard(
    centerpoint: Point<f64>,
    num_segments: usize,
    radii: Vec<f64>,
    boundary: Option<Polygon<f64>>,
) -> Vec<Polygon<f64>> {
    let mut polygons = Vec::new();
    let circle = makecircle(centerpoint, radii[0]);
    //println!("{:?}", circle);
    polygons.push(circle);
    //println!("{:?}", polygons[0]);
    polygons
}

pub fn makecircle(centerpoint: Point<f64>, radius: f64) -> Polygon<f64> {
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
        clockboard(Point::new(0.0, 0.0), 2, vec![1.0], None);
        assert!(false)
    }
}
