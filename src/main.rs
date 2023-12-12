use clap::Parser;
use gpx::{Gpx, GpxVersion, Track};
use std::error::Error;
use std::fs;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
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
            let trip = serde_json::from_str(&file_contents).unwrap(); //json!(file_contents);
                                                                      //println!("{:?}", &trip);
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

            let buffer = fs::File::create(args.output)?;
            gpx::write(&data, buffer).unwrap();
        }
    }

    Ok(())
}
