/// A single tidal harmonic constituent (e.g. M2, S2, K1, O1) for one
/// station, as published by a tide authority (NOAA, CHS, etc.).
///
/// These values are empirical, not computed — they come from the
/// station's harmonic constituent data, not derived at runtime.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct Constituent {
    /// Standard constituent name, e.g. "M2", "S2", "K1", "O1".
    pub name: String,
    /// Amplitude, in the station's reference units (typically meters).
    pub amplitude: f64,
    /// Phase lag, in degrees, relative to the constituent's
    /// equilibrium argument at the station's reference meridian.
    pub phase_degrees: f64,
}

impl Constituent {
    pub fn new(name: impl Into<String>, amplitude: f64, phase_degrees: f64) -> Self {
        Self {
            name: name.into(),
            amplitude,
            phase_degrees,
        }
    }
}
