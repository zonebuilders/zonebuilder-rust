use geo::Point;
use serde_json::to_string_pretty;
use zonebuilder::{clockboard, Params};
use structopt::StructOpt;

fn main() {

    // eprintln!("{:#?}", Params::default());

    let gj = clockboard(Point::new(0.0, 0.0), Params::from_args());
    // See https://github.com/georust/geojson/issues/161 for details
    let gjstring = to_string_pretty(&gj).unwrap();
    println!("{}", gjstring);
    // Write output if output provided
    // write!(&mut opt::output, "{}", gjstring);
}
