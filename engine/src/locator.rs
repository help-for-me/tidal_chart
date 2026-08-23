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
    /// distance, or `None` if `stations` is empty.
    ///
    /// Not yet implemented.
    pub fn nearest(&self, latitude: f64, longitude: f64) -> Option<&Station> {
        let _ = (latitude, longitude);
        unimplemented!("StationLocator::nearest not yet implemented")
    }
}
