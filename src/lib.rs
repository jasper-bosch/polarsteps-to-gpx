use anyhow::{Context, Result};
use geo_types::Point;
use gpx::{TrackSegment, Waypoint};

pub struct Route {
    pub track: TrackSegment,
}

impl Route {
    pub fn new(json: serde_json::Value) -> Result<Self> {
        let mut trkseg = TrackSegment::new();
        let locations = &json["locations"];
        if let Some(arr) = locations.as_array() {
            let mut points: Vec<Waypoint> = vec![];
            for point in arr {
                let mut new_point = Waypoint::new(Point::new(
                    point["lon"].as_f64().context("Can't parse longitude")?,
                    point["lat"].as_f64().context("Can't parse latitude")?,
                ));

                // Polarsteps stores timestamp as a Unix timestamp.
                let t: i64 = point["time"].as_f64().context("Can't parse time")? as i64;
                let timestamp = time::OffsetDateTime::from_unix_timestamp(t)?;
                new_point.time = Some(timestamp.into());

                points.push(new_point);
            }

            // Waypoints are not guaranteed to be in chronological order in the .json file.
            points.sort_by(|p1, p2| p1.time.cmp(&p2.time));

            trkseg.points = points;
        }

        Ok(Route { track: trkseg })
    }
}
