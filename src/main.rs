use geo::Point;

use zonebuilder::clockboard;

fn main() {
    let polygon_list = clockboard(Point::new(0.0, 0.0), 2, vec![1.0], None, ..Params::default());
    let geojson_list = geojson::Value::from(&polygon_list[0]);
    println!("{}", geojson_list);
}