# Roadmap

Versioned per [SemVer](https://semver.org/): starts at `0.0.0`, minor
bumps only after actually building and validating a real milestone (not
just planning one), `1.0.0` reserved for a fully working, stable
end-to-end app.

- **0.0.0** (current) — groundwork only: package layout, type shells,
  docs, CI. No working prediction logic.
- **0.1.0** — `HarmonicPredictor.waterLevel(at:)` implemented and
  validated against a real BC station's published predictions
  (regression test with known-correct output, not just "it compiles").
- **0.2.0** — BC station data sourced, converted, and loaded end to end;
  `StationLocator.nearest(toLatitude:longitude:)` implemented.
- **0.3.0** — Xcode app target scaffolded: SwiftUI + CoreLocation,
  depends on `TidalEngine`, shows a real tide chart for the nearest BC
  station, fully offline.
- **0.4.0** — Canada-wide station data (full CHS/DFO coverage).
- **0.5.0** — Canada + US station data (adds NOAA's ~3,400 stations).
- **0.6.0** — Global station data (TICON-4-derived), license checks
  resolved.
- **1.0.0** — stable, validated, real end-to-end use across the full
  data scope.

## Open questions blocking specific milestones

- **0.1.0**: none blocking — engine approach (original Swift
  implementation) and math reference (NOAA SP 98) are both decided.
- **0.2.0**: TCD-vs-direct-sourcing decision for the data pipeline (see
  `docs/ARCHITECTURE.md`); CHS/DFO license check.
- **0.3.0**: Xcode app-target scaffolding has to happen locally, not in
  this environment.
- **0.6.0**: TICON-4/GESLA license check.
