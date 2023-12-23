# polarsteps-to-gpx
Command-line program that converts a locations.json file from Polarsteps into a .gpx file. The coordinates along with their timestamp are sorted chronologically and then stored as a .gpx file.


## Usage
Two command-line arguments are required:

1. The path to the input file
1. The path to the output file

Example:

    cargo run -- --input "/home/user/locations.json" --output "/home/user/my-trip.gpx"
