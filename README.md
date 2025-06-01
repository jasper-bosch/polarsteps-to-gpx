# polarsteps-to-gpx
Command-line program that converts a locations.json file from [Polarsteps](https://www.polarsteps.com) to a .gpx file. The coordinates along with their timestamp are sorted chronologically and then stored as a .gpx file.

You can download a copy of your data from Polarsteps by following the steps on [this page](https://support.polarsteps.com/article/124-how-can-i-export-a-copy-of-my-data). The data for each trip is stored in a subfolder of the .zip file. Each of those subfolders contains a `locations.json` file with all the GPS coordinates of your trip.


## Usage
Two (optional) command-line arguments are:

1. The path to the input file. When this argument is not provided, the program will search for a file called `locations.json` in the current directory.
1. The path to the output file. When this argument is not provided, the program will simply replace the extension of the input file from `.json` to `.gpx` and use that.

Example:

    cargo run -- --input "/home/user/locations.json" --output "/home/user/my-trip.gpx"

You can also install the program by running `cargo install --path .` (or `just install` if you have [just](https://github.com/casey/just) installed). After that, you can run `polarsteps-to-gpx` in any folder. So you could open the folder of the trip you want to convert the `locations.json` file for in the terminal, and simply run `polarsteps-to-gpx`.
