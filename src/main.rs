use geo::Point;
use serde_json;
use serde_json::{to_string_pretty};
use zonebuilder::clockboard;
// use zonebuilder::clockpoly;
use zonebuilder::Params;

fn main() {

    // debug clockpoly 
    // let poly = clockpoly(Point::new(0.0, 0.0), 10.0, 1.0, 100, 12, 3);
    // let geojson = geojson::Value::from(&poly);
    // println!("{}", geojson);
    

    let gj = clockboard(Point::new(0.0, 0.0), Params::default());

    // Attempt to print pretty json - not outputting valid json currently
    // See https://github.com/georust/geojson/issues/161 for details
    // let geojson_list = geojson::Value::from(&polygon_list[0]);
    // let result = serde_json::to_string_pretty(&geojson_list);
    // println!("{}", result.unwrap());

    
    let gjstring = to_string_pretty(&gj).unwrap();

    // Which we can print / dump / etc:
    
    println!("{}", gjstring);

}
