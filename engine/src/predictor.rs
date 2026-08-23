use crate::{Station, TideExtremum};

/// Predicts water level at a station using the harmonic method: a sum
/// of cosine terms, one per constituent, each adjusted by a per-date
/// equilibrium argument and nodal factor.
///
/// Reference: NOAA Special Publication No. 98, "Manual of Harmonic
/// Analysis and Prediction of Tides" (public domain). This is a
/// from-scratch implementation of that published method — not derived
/// from XTide or any other GPL-licensed source — so the engine stays
/// permissively licensed and safe to embed anywhere.
pub struct HarmonicPredictor {
    pub station: Station,
}

impl HarmonicPredictor {
    pub fn new(station: Station) -> Self {
        Self { station }
    }

    /// Predicted water level at `unix_time_seconds`, relative to
    /// `station.datum_offset`.
    ///
    /// Not yet implemented. This is where the constituent summation
    /// (per-date equilibrium argument + nodal factor, per SP 98) goes
    /// once the buildout starts.
    pub fn water_level(&self, unix_time_seconds: i64) -> f64 {
        let _ = unix_time_seconds;
        unimplemented!("HarmonicPredictor::water_level not yet implemented")
    }

    /// High and low tide events (local extrema of [`Self::water_level`])
    /// between `start_unix_seconds` and `end_unix_seconds`, in
    /// chronological order.
    ///
    /// The summed curve is a sum of cosines with different periods, so
    /// extrema have no closed form — this requires sampling
    /// `water_level` and refining around sign changes in its slope
    /// (e.g. bisection or Newton's method on a numerical derivative),
    /// not a formula.
    ///
    /// Not yet implemented.
    pub fn extrema(&self, start_unix_seconds: i64, end_unix_seconds: i64) -> Vec<TideExtremum> {
        let _ = (start_unix_seconds, end_unix_seconds);
        unimplemented!("HarmonicPredictor::extrema not yet implemented")
    }
}
