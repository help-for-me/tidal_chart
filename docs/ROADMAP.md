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

- **0.0.0** — `engine/`'s harmonic-prediction math implemented and
  self-consistency tested (constituent speeds match independently
  published invariant constants; a synthetic station's extrema/
  water-level agree with each other); several formulas checked directly
  against Schureman's primary 1958 text.
- **0.1.0 (current)** — validated against **8 real CHS stations**
  spanning BC, the Arctic, and the Atlantic/Gulf coast (Point Atkinson,
  Port Hardy, Daajing Giids, Vancouver, Ulukhaktok, Yarmouth,
  Cap-aux-Meules, Sept-Îles), using real observed water-level time
  series (1 month to 7+ years each). Methodology: fit harmonic constants
  from 80% of each series (a real harmonic *analysis*, not synthetic
  data), predict the held-out 20%, and check both (a) this engine
  matches an independent Python evaluation of the same fit exactly, and
  (b) the prediction tracks the real, unseen observations to within
  8–38cm RMS — consistent with ordinary weather-driven noise (wind,
  atmospheric pressure) a pure harmonic model doesn't capture, not
  evidence of engine error. Raw station data and full per-station
  results live in a private companion repo (not public — CHS's data
  license isn't flatly public domain; see `docs/VALIDATION.md`). **Note
  what this does and doesn't prove**: it confirms the engine correctly
  reproduces a real, independently-fitted harmonic model of real tides
  to a physically-sensible tolerance — it's *not* the same as matching
  CHS's own official published prediction tables number-for-number
  (which needs CHS's own harmonic constants directly, still not
  obtained — see `docs/VALIDATION.md`), a distinct, lower-priority
  remaining check given how strong this evidence already is.
- **0.2.0 (current)** — `engine::parse_station()` reads a simple,
  dependency-free text format into a `Station` (`engine/src/data.rs`) —
  the app-consumable loading half of the pipeline, as real code, not
  just a validation script; verified round-tripping actual fitted data
  through it end to end (parse → `HarmonicPredictor` → sensible
  alternating High/Low extrema). Four real BC stations (Point Atkinson,
  Port Hardy, Vancouver, Daajing Giids) are bundled in this format —
  **for personal use, in the private companion repo, not this one**
  (per the same CHS licensing/non-commercial constraint as 0.1.0's raw
  data — see `docs/VALIDATION.md`). Three of the four were fit from
  only ~30-day series, so only 12 of 23 constituents could be resolved
  (documented, not hidden, in the private repo) — real, working, but
  not CHS-official prediction quality. `StationLocator::nearest`
  against real bundled data, and a proper multi-region distributable
  pipeline (once licensing allows), remain open — see below.

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
  override to GPS). **Reopens a timezone question deferred from the
  MVP**: MVP shows all times in the device's local timezone, which is a
  non-issue there because the viewer and the station are always the
  same place (nearest-station lookup ties them together). Once a
  station can be chosen far from the viewer, "local" becomes ambiguous
  — the station's own local time (what a tide table conventionally
  shows) vs. the viewer's device time are no longer the same thing, and
  which one to display isn't decided yet. Engine output itself isn't
  affected (`unix_time_seconds` is already true UTC throughout,
  confirmed during the real-station validation above — this is purely a
  display-layer decision for whenever this milestone is reached).
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

- **0.9.0**: viewer-local vs. station-local time display for a
  non-nearest station — undecided, see above.
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
