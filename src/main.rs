use geo::Point;
use zonebuilder::clockboard;
use zonebuilder::Params;

fn main() {
    let polygon_list = clockboard(Point::new(0.0, 0.0), Params::default(), None);
    let geojson_list = geojson::Value::from(&polygon_list);
    println!("{}", geojson_list);
}
