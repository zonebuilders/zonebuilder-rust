use geo::Point;
use zonebuilder::{clockboard, Params};
use clap::{Arg, App};

fn main() {
    let clockboard: clap::ArgMatches = App::new("My Test Program")
    .version("0.1.0")
    .author("zonebuilders")
    .about("Builds zones")
    .arg(Arg::with_name("num_vertices")
             .short("n")
             .long("number")
             .takes_value(true)
             .help("Number of vertices"))
    .clockboard();

    let num_vertices = matches.value_of("num_vertices").unwrap_or("input.txt");

    let polygon_list = clockboard(Point::new(0.0, 0.0), Params::default(), None);
    let geojson_list = geojson::Value::from(&polygon_list[0]);
    println!("{}", geojson_list);
}