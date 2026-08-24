# Roadmap

Versioned per [SemVer](https://semver.org/): starts at `0.0.0`, minor
bumps only after actually building and validating a real milestone (not
just planning one), `1.0.0` reserved for a fully working, stable
end-to-end app.

Platform order: **iOS → Linux → macOS → Android → Windows → Garmin.**
Linux, macOS, and Windows share one codebase (the `egui` desktop app —
see `docs/ARCHITECTURE.md`), so those three milestones are mostly about
validating on that OS, not separate builds from scratch. Garmin is a
distinct case — see its milestone note below.

- **0.0.0** (current) — `engine/`'s harmonic-prediction math
  (`HarmonicPredictor::water_level`, `::extrema`) and
  `StationLocator::nearest` are implemented and self-consistency
  tested (constituent speeds match independently published invariant
  constants; a synthetic station's extrema/water-level agree with each
  other). **Not yet bumped to 0.1.0/0.2.0** — per this repo's own
  versioning rule, a real milestone needs *validating*, not just
  building, and that validation (a real station's published predictions
  checked against this engine's output) hasn't happened yet. See
  `docs/VALIDATION.md` for exactly what's checked vs. not, and why
  (government tide-data domains are blocked by this environment's
  network policy — a real, confirmed constraint, not an oversight).
- **0.1.0** — `HarmonicPredictor::water_level` validated against a real
  BC station's published predictions (regression test with
  known-correct output from a real government source, not just
  internal self-consistency).
- **0.2.0** — real BC station data sourced, converted, and loaded end
  to end (the data *pipeline*, not just the math) — `extrema` and
  `nearest` validated against that same real data.

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
  Windows — completes platform parity across the five app-store/desktop
  targets.
- **0.8.0 — Garmin MVP.** Connect IQ (Monkey C) watch app. Different
  shape from the others: Garmin's Connect IQ platform doesn't run Rust,
  so `engine/` can't be linked or bound to it the way the other four
  platforms do — see `docs/ARCHITECTURE.md` for the two real options
  (phone-computed data synced to the watch, vs. a from-scratch Monkey C
  port of the engine) and which this defaults to.

### Post-MVP features (applied across platforms as each is reached)

- **0.9.0** — change location: choose a station/place manually (already
  required as the desktop MVP's baseline; extends it to mobile as an
  override to GPS).
- **0.10.0** — change date: view the tide window centered on a different
  date.
- **0.11.0** — configure the range of days shown (the MVP's fixed 5-day
  window becomes adjustable).
- **0.12.0** — full tidal graph: a continuous predicted water-level
  curve, not just high/low points.
- **0.13.0** — sunrise/sunset times and moon phase, shown alongside the
  tide chart. Moon phase and tides are genuinely connected (new/full
  moon — syzygy — means spring tides; quarter moons mean neap tides,
  from the same sun-moon-earth alignment that drives both), and moon
  phase can reuse `engine/`'s existing sun/moon mean-longitude elements
  rather than a separate calculation — see `docs/ARCHITECTURE.md`.

### Data coverage expansion

Each stage needs its own real-data validation pass per
`docs/VALIDATION.md` — the government authority changes per stage, so
"validated" for BC doesn't carry over automatically.

- **0.14.0** — Canada-wide station data (full CHS/DFO coverage),
  validated against CHS.
- **0.15.0** — Canada + US station data (adds NOAA's ~3,400 stations),
  validated against NOAA CO-OPS.
- **0.16.0** — Global station data (TICON-4-derived), license checks
  resolved, validated against whichever national hydrographic offices
  publish checkable predictions for the stations actually bundled (see
  `docs/VALIDATION.md`'s "Global stage note").
- **1.0.0** — stable, validated, real end-to-end use across the full
  data scope and feature set above, on all six platforms.

## Open questions blocking specific milestones

- **0.1.0–0.2.0**: need a real BC station's harmonic constants + real
  published predictions from CHS to validate against — blocked by this
  environment's network policy; can be unblocked by the user fetching
  and pasting that data in, or by a future session with broader network
  access.
- **0.3.0**: binding generation (UniFFI or alternative) needs
  evaluating; Xcode app-target scaffolding has to happen locally, not in
  this environment.
- **0.4.0–0.5.0, 0.7.0**: none — the desktop app is buildable in an
  environment like this one, unlike the mobile shells.
- **0.6.0**: Android Studio app-module scaffolding has to happen
  locally.
- **0.8.0**: phone-syncs-to-watch vs. Monkey C engine port — needs
  deciding before Garmin work starts (see `docs/ARCHITECTURE.md`).
- **0.16.0**: TICON-4/GESLA license check; per-country validation-source
  survey.
