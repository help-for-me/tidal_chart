//! `tidal_engine`: offline, on-device tide prediction via the harmonic
//! method (NOAA Special Publication No. 98, public domain).
//!
//! Written from scratch against the published formulas, not derived
//! from XTide or any other GPL-licensed source, so this crate stays
//! permissively licensed (MIT OR Apache-2.0) and safe to embed
//! anywhere: App Store, Play Store, other people's projects.
//!
//! Pure computation, no platform dependencies (no networking, no I/O,
//! no UI) — meant to be consumed as a module from other projects,
//! either directly as a Rust crate or through generated bindings for
//! Swift/Kotlin. See `docs/ARCHITECTURE.md` in the repo root for the
//! binding strategy.

mod constituent;
mod extremum;
mod locator;
mod predictor;
mod station;

pub use constituent::Constituent;
pub use extremum::{TideExtremum, TideExtremumKind};
pub use locator::StationLocator;
pub use predictor::HarmonicPredictor;
pub use station::Station;
