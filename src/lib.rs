use anyhow::{Context, Result};
use geo_types::Point;
use gpx::{TrackSegment, Waypoint};

pub struct Route {
    pub track: TrackSegment,
}

impl Route {
    /// # Errors
    /// - If latitude, longitude, or time cannot be parsed from the JSON.
    pub fn new(json: &serde_json::Value) -> Result<Self> {
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
                #[allow(clippy::cast_possible_truncation)]
                let t = point["time"]
                    .as_f64()
                    .context(format!("Can't parse time as f64: {}", point["time"]))?
                    as i64;
                let timestamp = time::OffsetDateTime::from_unix_timestamp(t)?;
                new_point.time = Some(timestamp.into());

                points.push(new_point);
            }

            // Waypoints are not guaranteed to be in chronological order in the .json file.
            points.sort_by_key(|p| p.time);

            trkseg.points = points;
        }

        Ok(Route { track: trkseg })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn timestamps(route: &Route) -> Vec<i64> {
        route
            .track
            .points
            .iter()
            .map(|p| {
                let t: time::OffsetDateTime = p.time.unwrap().into();
                t.unix_timestamp()
            })
            .collect()
    }

    #[test]
    fn sorts_out_of_order_waypoints_chronologically() {
        let input = json!({
            "locations": [
                { "lat": 52.0, "lon": 4.0, "time": 1_000_000 },
                { "lat": 53.0, "lon": 5.0, "time":   500_000 },
                { "lat": 54.0, "lon": 6.0, "time": 1_500_000 },
            ]
        });

        let route = Route::new(&input).unwrap();

        assert_eq!(timestamps(&route), vec![500_000, 1_000_000, 1_500_000]);
    }

    #[test]
    fn parses_float_timestamps() {
        // Polarsteps sometimes stores timestamps as floats (e.g. 1234567890.5).
        // The fractional part is truncated to i64 — the key requirement is that
        // parsing succeeds rather than returning an error.
        let input = json!({
            "locations": [
                { "lat": 52.0, "lon": 4.0, "time": 1_000_000.9 },
                { "lat": 53.0, "lon": 5.0, "time":   500_000.1 },
            ]
        });

        let route = Route::new(&input).unwrap();

        assert_eq!(timestamps(&route), vec![500_000, 1_000_000]);
    }
}
