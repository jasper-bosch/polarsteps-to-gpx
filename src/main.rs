//! Command-line program that converts a locations.json file from Polarsteps
//! into a .gpx file.
use std::{
    fs,
    io::{BufWriter, Write},
    path::PathBuf,
};

use anyhow::{Context, Result};
use clap::Parser;
use gpx::{Gpx, GpxVersion, Track};

#[derive(Parser, Debug)]
struct Args {
    /// Path to the input locations.json file
    #[arg(short, long, default_value = "locations.json")]
    input: PathBuf,

    /// Path to the output .gpx file [default: the input path with a .gpx extension]
    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let file_contents =
        fs::read_to_string(&args.input).with_context(|| args.input.display().to_string())?;
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

    let output_path = args
        .output
        .unwrap_or_else(|| args.input.with_extension("gpx"));

    let mut buffer = BufWriter::new(
        fs::File::create(&output_path).with_context(|| output_path.display().to_string())?,
    );
    gpx::write(&data, &mut buffer).with_context(|| output_path.display().to_string())?;
    buffer
        .flush()
        .with_context(|| output_path.display().to_string())?;
    println!("{point_count} points written to {}", output_path.display());

    Ok(())
}
