use crate::Station;

/// Finds the nearest station to a GPS coordinate, from whatever
/// station data is loaded.
pub struct StationLocator {
    pub stations: Vec<Station>,
}

impl StationLocator {
    pub fn new(stations: Vec<Station>) -> Self {
        Self { stations }
    }

    /// Nearest station to (`latitude`, `longitude`) by great-circle
    /// (haversine) distance, or `None` if `stations` is empty.
    pub fn nearest(&self, latitude: f64, longitude: f64) -> Option<&Station> {
        self.stations.iter().min_by(|a, b| {
            haversine_km(latitude, longitude, a.latitude, a.longitude).total_cmp(&haversine_km(
                latitude,
                longitude,
                b.latitude,
                b.longitude,
            ))
        })
    }
}

fn haversine_km(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    let (lat1_r, lat2_r) = (lat1.to_radians(), lat2.to_radians());
    let d_lat = (lat2 - lat1).to_radians();
    let d_lon = (lon2 - lon1).to_radians();
    let a = (d_lat / 2.0).sin().powi(2) + lat1_r.cos() * lat2_r.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    EARTH_RADIUS_KM * c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Station;

    #[test]
    fn nearest_picks_the_closer_of_two_stations() {
        let vancouver = Station::new("van", "Vancouver", 49.28, -123.12, 0.0, vec![]);
        let victoria = Station::new("vic", "Victoria", 48.43, -123.37, 0.0, vec![]);
        let locator = StationLocator::new(vec![vancouver, victoria]);

        // A point just north of Vancouver should find Vancouver, not Victoria.
        let nearest = locator.nearest(49.5, -123.1).unwrap();
        assert_eq!(nearest.id, "van");
    }

    #[test]
    fn nearest_is_none_when_no_stations() {
        let locator = StationLocator::new(vec![]);
        assert!(locator.nearest(49.28, -123.12).is_none());
    }
}
