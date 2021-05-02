// See https://github.com/zonebuilders/zonebuilder-rust/issues/23
use geo::Point;
use serde_json::to_string_pretty;
use structopt::StructOpt;
use zonebuilder::Params;

fn main() {
    let cp = Point::new(-1.5, 53.8);
    let gj = zonebuilder::clockboard(cp, Params::from_args());
    let gjstring = to_string_pretty(&gj).unwrap();
    println!("{}", gjstring);
}
