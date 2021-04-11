use geo::Point;
use serde_json::to_string_pretty;
use zonebuilder::{clockboard, Params};
// use std::path::PathBuf;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "zb")]
pub struct Opt {
    // // A flag, true if used in the command line. Note doc comment will
    // // be used for the help message of the flag. The name of the
    // // argument will be, by default, based on the name of the field.
    // /// Activate debug mode
    // #[structopt(long)]
    // debug: bool,

    // // The number of occurrences of the `v/verbose` flag
    // /// Verbose mode (-v, -vv, -vvv, etc.)
    // #[structopt(long, parse(from_occurrences))]
    // verbose: u8,

    /// Set n_circles
    #[structopt(short = "c", long, default_value = "5")]
    n_circles: usize,

    /// Number of radial segments (12 by default)
    #[structopt(short = "s", long, default_value = "12")]
    num_segments: usize,

    /// Distances between concentric rings.
    /// First 5 values of the triangular number sequence (1 to 15)
    /// by default
    #[structopt(short, long, default_value = "vec![1.0, 3.0, 6.0, 10.0, 15.0]")]
    distances: Vec<f64>,

    /// Number of vertices per arc
    #[structopt(short = "v", long, default_value = "5")]
    num_vertices_arc: usize,

    /// Number of decimal places in the resulting output (GeoJSON) files.
    /// Set to 6 by default. Larger numbers mean more precision but
    /// larger file sizes.
    #[structopt(short, long, default_value = "5")]
    precision: usize,

    // /// Output file
    // #[structopt(short, long)]
    // output: PathBuf,

}


fn main() {

    let opt = Opt::from_args();
    eprintln!("{:#?}", &opt);

    let gj = clockboard(Point::new(0.0, 0.0), opt);
    // See https://github.com/georust/geojson/issues/161 for details
    let gjstring = to_string_pretty(&gj).unwrap();
    println!("{}", gjstring);
    // Write output if output provided
    // write!(&mut opt::output, "{}", gjstring);
}
