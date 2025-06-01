//! Command-line program that converts a locations.json file from Polarsteps
//! into a .gpx file.
use clap::Parser;
use gpx::{Gpx, GpxVersion, Track};
use std::error::Error;
use std::fs;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
struct Args {
    // Path to input locations.json file. Default to current directory.
    #[arg(short, long, default_value = "locations.json")]
    input: String,

    // Path to output .gpx file (optional).
    // If not provided, it will default to "locations.gpx" in the current directory.
    #[arg(short, long, default_value = "")]
    output: String,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> MyResult<()> {
    match fs::read_to_string(&args.input) {
        Err(e) => eprintln!("{}: {}", args.input, e),
        Ok(file_contents) => {
            let trip = serde_json::from_str(&file_contents).unwrap();
            let route = polarsteps_to_gpx::Route::new(trip);
            let track = Track {
                segments: vec![route.track],
                ..Default::default()
            };

            let data = Gpx {
                version: GpxVersion::Gpx11,
                creator: Some("polarsteps-to-gpx".to_string()),
                tracks: vec![track],
                ..Default::default()
            };

            let output_path = if args.output.is_empty() {
                args.input.replace(".json", ".gpx")
            } else {
                args.output
            };

            let buffer = fs::File::create(output_path)?;
            gpx::write(&data, buffer).unwrap();
        }
    }

    Ok(())
}
