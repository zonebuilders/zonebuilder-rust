use geo::Point;
// use geojson::to_string_pretty;
use zonebuilder::clockboard;
use zonebuilder::Params;

fn main() {
    let polygon_list = clockboard(Point::new(0.0, 0.0), Params::default());

    // This works but only restults in 1 polygon:
    let geojson_list = geojson::Value::from(&polygon_list[0]);
    println!("{}", geojson_list);

    // This fails - cannot handle more than 1 polygon it seems and there's no unwrap method I can see:
    // let geojson_list = geojson::Value::from(&polygon_list);
    // println!("{}", geojson_list);
    
    // serde json version gives:     [  [   [   1.0,  0.0   ], (no 'geo')
    // let result = serde_json::to_string_pretty(&geojson_list);
    // println!("{}", result.unwrap());

    // Trying to get it working with json  to_string_pretty:
    // let geojson_string = polygon_list.to_string_pretty();
    // let geojson_unwrapped = polygon_list.unwrap()
    // println!("{}", geojson_unwrapped);

}
