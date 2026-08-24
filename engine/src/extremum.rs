/// Whether a [`TideExtremum`] is a high or low tide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, uniffi::Enum)]
pub enum TideExtremumKind {
    High,
    Low,
}

/// A single high or low tide event: a local extremum of the predicted
/// water level curve.
#[derive(Debug, Clone, PartialEq, uniffi::Record)]
pub struct TideExtremum {
    pub kind: TideExtremumKind,
    /// Unix timestamp, seconds. Turned out to be sufficient on its own —
    /// `astro::julian_day` derives Julian Day directly from it, no
    /// calendar-aware date type or extra dependency needed.
    pub unix_time_seconds: i64,
    /// Predicted water level at this time, relative to the station's
    /// `datum_offset` — same units as [`crate::Constituent::amplitude`].
    pub height: f64,
}
