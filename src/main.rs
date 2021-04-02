use geo::Point;
use serde_json::to_string_pretty;
use zonebuilder::{clockboard, Params};

fn main() {
    let gj = clockboard(Point::new(0.0, 0.0), Params::default());
    // See https://github.com/georust/geojson/issues/161 for details
    let gjstring = to_string_pretty(&gj).unwrap();
    println!("{}", gjstring);
}
