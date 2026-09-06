//! Command-line program that converts a locations.json file from Polarsteps
//! into a .gpx file.
use std::{
    fs,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::{Context, Result};
use clap::Parser;
use gpx::{Gpx, GpxVersion, Track};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input locations.json file
    #[arg(short, long, default_value = "locations.json")]
    input: String,

    /// Path to the output .gpx file [default: the input path with a .gpx extension]
    #[arg(short, long, default_value = "", hide_default_value = true)]
    output: String,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let file_contents = fs::read_to_string(&args.input).with_context(|| args.input.clone())?;
    let trip = &serde_json::from_str(&file_contents)?;
    let route = polarsteps_to_gpx::Route::new(trip)?;
    let point_count = route.track.points.len();
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
        Path::new(&args.input)
            .with_extension("gpx")
            .to_string_lossy()
            .into_owned()
    } else {
        args.output
    };

    let mut buffer = BufWriter::new(fs::File::create(&output_path)?);
    gpx::write(&data, &mut buffer)?;
    buffer.flush()?;
    println!("{point_count} points written to {output_path}");

    Ok(())
}
