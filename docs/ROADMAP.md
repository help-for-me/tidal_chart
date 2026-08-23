# Roadmap

Versioned per [SemVer](https://semver.org/): starts at `0.0.0`, minor
bumps only after actually building and validating a real milestone (not
just planning one), `1.0.0` reserved for a fully working, stable
end-to-end app.

- **0.0.0** (current) — groundwork only: `engine/` crate layout, type
  shells, docs, CI. No working prediction logic.
- **0.1.0** — `HarmonicPredictor::water_level` implemented and validated
  against a real BC station's published predictions (regression test
  with known-correct output, not just "it builds").
- **0.2.0** — BC station data sourced, converted, and loaded end to end;
  `StationLocator::nearest` and `HarmonicPredictor::extrema` (high/low
  tide finding) implemented.
- **0.3.0 — iOS MVP** — Xcode app target scaffolded: SwiftUI +
  CoreLocation, depends on `engine/` via generated Swift bindings. On
  open, shows the device's current coordinates and the current time and
  water level, then a chart of high/low tide for the nearest BC station
  across a 5-day window (yesterday, today, next 3 days). Fully offline.

### Post-MVP features

- **0.4.0** — change location: override GPS with a manually chosen
  station/place.
- **0.5.0** — change date: view the tide window centered on a different
  date.
- **0.6.0** — configure the range of days shown (the MVP's fixed 5-day
  window becomes adjustable).
- **0.7.0** — full tidal graph: a continuous predicted water-level
  curve, not just high/low points.
- **0.8.0** — Android app: Kotlin + Jetpack Compose, depends on
  `engine/` via generated Kotlin bindings, same MVP feature set as iOS.

### Data coverage expansion

- **0.9.0** — Canada-wide station data (full CHS/DFO coverage).
- **0.10.0** — Canada + US station data (adds NOAA's ~3,400 stations).
- **0.11.0** — Global station data (TICON-4-derived), license checks
  resolved.
- **1.0.0** — stable, validated, real end-to-end use across the full
  data scope and feature set above, on both platforms.

## Open questions blocking specific milestones

- **0.1.0**: none blocking — engine approach (original Rust
  implementation) and math reference (NOAA SP 98) are both decided.
- **0.2.0**: TCD-vs-direct-sourcing decision for the data pipeline (see
  `docs/ARCHITECTURE.md`); CHS/DFO license check.
- **0.3.0**: binding generation (UniFFI or alternative) needs
  evaluating; Xcode app-target scaffolding has to happen locally, not in
  this environment.
- **0.8.0**: Android Studio app-module scaffolding has to happen
  locally.
- **0.11.0**: TICON-4/GESLA license check.
