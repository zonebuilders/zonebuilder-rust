use geo::Point;
use serde_json::to_string_pretty;
use structopt::StructOpt;
use zonebuilder::{clockboard, Params};

/// Command line options: where the clockboard sits, then the zone parameters.
#[derive(StructOpt, Debug)]
struct Cli {
    /// Longitude of the centre (x if --projected).
    #[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
    lon: f64,

    /// Latitude of the centre (y if --projected).
    #[structopt(long, default_value = "0.0", allow_hyphen_values = true)]
    lat: f64,

    #[structopt(flatten)]
    params: Params,
}

fn main() {
    let cli = Cli::from_args();
    let gj = clockboard(Point::new(cli.lon, cli.lat), cli.params);
    // See https://github.com/georust/geojson/issues/161 for details
    let gjstring = to_string_pretty(&gj).unwrap();
    println!("{}", gjstring);
}
