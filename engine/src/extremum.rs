/// Whether a [`TideExtremum`] is a high or low tide.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TideExtremumKind {
    High,
    Low,
}

/// A single high or low tide event: a local extremum of the predicted
/// water level curve.
#[derive(Debug, Clone, PartialEq)]
pub struct TideExtremum {
    pub kind: TideExtremumKind,
    /// Unix timestamp, seconds.
    ///
    /// TODO: move to a calendar-aware date type once one is chosen for
    /// the equilibrium-argument/nodal-factor math (see `predictor.rs`),
    /// which needs real calendar dates, not just an instant. Left as a
    /// plain timestamp for now to keep this crate dependency-free.
    pub unix_time_seconds: i64,
    /// Predicted water level at this time, relative to the station's
    /// `datum_offset` — same units as [`crate::Constituent::amplitude`].
    pub height: f64,
}
