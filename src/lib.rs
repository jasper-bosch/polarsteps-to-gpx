use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use geo_types::Point;
use gpx::{Time, TrackSegment, Waypoint};

pub struct Route {
    pub track: TrackSegment,
}

impl Route {
    pub fn new(json: serde_json::Value) -> Self {
        let mut trkseg = TrackSegment::new();
        let locations = &json["locations"];
        if let Some(arr) = locations.as_array() {
            for point in arr {
                let mut new_point = Waypoint::new(Point::new(
                    point["lat"].as_f64().unwrap(),
                    point["lon"].as_f64().unwrap(),
                ));

                let t: i64 = point["time"].as_f64().unwrap() as i64;
                let timestamp = time::OffsetDateTime::from_unix_timestamp(t).unwrap();
                new_point.time = Some(timestamp.into());

                trkseg.points.push(new_point);
            }
        }

        Route { track: trkseg }
    }
}
