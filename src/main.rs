use geo::Point;
use zonebuilder::Params;
use zonebuilder::clockboard;

fn main() {
    let polygon_list = clockboard(Point::new(0.0, 0.0), Params::default(), None);
    let geojson_list = geojson::Value::from(&polygon_list[0]);
    println!("{:.5}", geojson_list.);
}