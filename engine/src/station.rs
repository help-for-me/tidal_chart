use crate::Constituent;

/// A tide station: location plus the harmonic constituents needed to
/// predict water level there.
#[derive(Debug, Clone, PartialEq)]
pub struct Station {
    /// Stable identifier from the source dataset (e.g. NOAA/CHS station ID).
    pub id: String,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    /// Height (same units as each constituent's amplitude) that
    /// predictions are measured relative to, e.g. Mean Lower Low Water.
    pub datum_offset: f64,
    pub constituents: Vec<Constituent>,
}

impl Station {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        latitude: f64,
        longitude: f64,
        datum_offset: f64,
        constituents: Vec<Constituent>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            latitude,
            longitude,
            datum_offset,
            constituents,
        }
    }
}
