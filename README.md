# polarsteps-to-gpx
Command-line program that converts a locations.json file from [Polarsteps](https://www.polarsteps.com) to a .gpx file. The coordinates along with their timestamp are sorted chronologically and then stored as a .gpx file.

You can download a copy of your data from Polarsteps by following the steps on [this page](https://support.polarsteps.com/article/124-how-can-i-export-a-copy-of-my-data). The data for each trip is stored in a subfolder of the .zip file. Each of those subfolders contains a `locations.json` file with all the GPS coordinates of your trip.


## Usage
Two command-line arguments are required:

1. The path to the input file
1. The path to the output file

Example:

    cargo run -- --input "/home/user/locations.json" --output "/home/user/my-trip.gpx"
