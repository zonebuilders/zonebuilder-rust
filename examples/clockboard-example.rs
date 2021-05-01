// See https://github.com/zonebuilders/zonebuilder-rust/issues/23
extern crate zonebuilder;
use zonebuilder::Params;
use structopt::StructOpt;
use geo::Point;
use serde_json::to_string_pretty;

fn main() {
    let cp = Point::new(-1.5, 53.8);
    let gj = zonebuilder::clockboard(cp, Params::from_args());
    let gjstring = to_string_pretty(&gj).unwrap();
    println!("{}", gjstring);
}