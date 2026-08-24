# Roadmap

Versioned per [SemVer](https://semver.org/): starts at `0.0.0`, minor
bumps only after actually building and validating a real milestone (not
just planning one), `1.0.0` reserved for a fully working, stable
end-to-end app.

Platform order: **iOS → Linux → macOS → Android → Windows.** Linux,
macOS, and Windows share one codebase (the `egui` desktop app — see
`docs/ARCHITECTURE.md`), so those three milestones are mostly about
validating on that OS, not separate builds from scratch.

- **0.0.0** (current) — groundwork only: `engine/` crate layout, type
  shells, docs, CI. No working prediction logic.
- **0.1.0** — `HarmonicPredictor::water_level` implemented and validated
  against a real BC station's published predictions (regression test
  with known-correct output, not just "it builds").
- **0.2.0** — BC station data sourced, converted, and loaded end to end;
  `StationLocator::nearest` and `HarmonicPredictor::extrema` (high/low
  tide finding) implemented.

### Platform MVPs, in order

Each ships the same MVP screen: current location (GPS on mobile, manual
entry on desktop) + current time and water level, then a 5-day (-1/+3)
high/low tide chart for the nearest BC station, fully offline.

- **0.3.0 — iOS MVP.** Xcode app target, SwiftUI + CoreLocation, depends
  on `engine/` via generated Swift bindings.
- **0.4.0 — Linux MVP.** `egui`/`eframe` desktop app built and validated
  on Linux; manual location entry (no GPS on desktop).
- **0.5.0 — macOS MVP.** Same desktop app codebase, validated on macOS.
- **0.6.0 — Android MVP.** Kotlin + Jetpack Compose, depends on
  `engine/` via generated Kotlin bindings.
- **0.7.0 — Windows MVP.** Same desktop app codebase, validated on
  Windows — completes platform parity across all five targets.

### Post-MVP features (applied across platforms as each is reached)

- **0.8.0** — change location: choose a station/place manually (already
  required as the desktop MVP's baseline; extends it to mobile as an
  override to GPS).
- **0.9.0** — change date: view the tide window centered on a different
  date.
- **0.10.0** — configure the range of days shown (the MVP's fixed 5-day
  window becomes adjustable).
- **0.11.0** — full tidal graph: a continuous predicted water-level
  curve, not just high/low points.

### Data coverage expansion

- **0.12.0** — Canada-wide station data (full CHS/DFO coverage).
- **0.13.0** — Canada + US station data (adds NOAA's ~3,400 stations).
- **0.14.0** — Global station data (TICON-4-derived), license checks
  resolved.
- **1.0.0** — stable, validated, real end-to-end use across the full
  data scope and feature set above, on all five platforms.

## Open questions blocking specific milestones

- **0.1.0**: none blocking — engine approach (original Rust
  implementation) and math reference (NOAA SP 98) are both decided.
- **0.2.0**: TCD-vs-direct-sourcing decision for the data pipeline (see
  `docs/ARCHITECTURE.md`); CHS/DFO license check.
- **0.3.0**: binding generation (UniFFI or alternative) needs
  evaluating; Xcode app-target scaffolding has to happen locally, not in
  this environment.
- **0.4.0–0.5.0, 0.7.0**: none — the desktop app is buildable in an
  environment like this one, unlike the mobile shells.
- **0.6.0**: Android Studio app-module scaffolding has to happen
  locally.
- **0.14.0**: TICON-4/GESLA license check.
