# Tidal Chart

Offline-first tide-prediction app targeting **iOS, Linux, macOS,
Android, and Windows** (built and validated in that order): GPS (or,
where GPS isn't available, a manually entered location) finds the
nearest tide station, then a local harmonic-method engine computes the
prediction from bundled constituent data. No network dependency for the
core tide-chart flow, no server-fetched predictions.

- Status: planning
- Version: 0.0.0 (nothing built/tested yet)

## Why on-device computation, not cached predictions

Plenty of "offline" tide apps just cache a pre-fetched date range from a
server and stop working once it runs out, or wherever it never fetched
for. This app instead does the actual harmonic computation locally, the
same method (and public-domain formulas) national tide authorities use
to publish predictions in the first place. Once station data is bundled,
it works anywhere, indefinitely, with zero connectivity.

## Platform order

**iOS first** — that's where active development starts. Everything
below it is built and validated in this order once iOS is working:

1. **iOS**
2. **Linux**
3. **macOS**
4. **Android**
5. **Windows**

Every tool and architecture decision (the engine, the data pipeline, the
UI approach) is made with all five in mind from the start, not
iOS-only decisions that get retrofitted later. See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for how each layer stays
portable.

## Architecture at a glance

- **Engine**: `engine/` — a single, original Rust implementation of the
  harmonic prediction method (public-domain formulas from NOAA Special
  Publication No. 98), not a GPL dependency. Rust so one implementation
  is shared across every platform above — natively on desktop, via
  generated bindings on mobile — rather than reimplementing the math
  per platform. See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for
  the reasoning.
- **Data**: harmonic constituent data per tide station, staged by region
  (see below), converted to an open format the engine reads. See
  [`docs/DATA.md`](docs/DATA.md).
- **iOS app**: SwiftUI + CoreLocation, consuming `engine/` through
  generated Swift bindings. Not yet scaffolded — first thing to build.
- **Android app**: Kotlin + Jetpack Compose, consuming `engine/` through
  generated Kotlin bindings. Not yet scaffolded.
- **Desktop app** (Linux/macOS/Windows): one shared Rust GUI (`egui`),
  linking `engine/` directly — no bindings needed since both are Rust.
  One codebase, validated on each OS in the order above. Not yet
  scaffolded.

None of the four app shells are scaffolded yet — all need their
respective toolchains locally (Xcode, Android Studio) except the
desktop app, which can be built in this kind of environment. See
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#app-shells-next-step) for
what's next on each.

## MVP screen

On open: show the current location (coordinates via GPS on mobile;
manually entered on desktop, which has no GPS) and the current time and
water level, then a chart of high and low tide for the nearest station
across a 5-day window — yesterday, today, and the next 3 days.

## Planned features (post-MVP)

In rough priority order, applied across platforms as each is reached —
see [`docs/ROADMAP.md`](docs/ROADMAP.md) for how these map to versions:

1. Change location (override GPS with a manually chosen station/place —
   already required as a baseline on desktop, this extends it to
   mobile).
2. Change date (view the tide window centered on a different date).
3. Configure the range of days shown (the MVP's 5-day window becomes
   adjustable).
4. Full tidal graph — a continuous water-level curve, not just
   high/low points.

## Data scope roadmap

Starting narrow and expanding, matching the engine/app maturing
alongside it rather than trying to solve global coverage on day one:

1. **BC** (British Columbia, Canada) — first target, smallest scope to
   validate the full pipeline end to end.
2. **Canada** — expand to full CHS/DFO coverage.
3. **Canada + US** — add NOAA's ~3,400 public-domain stations.
4. **Global** — broaden to full worldwide coverage (e.g. TICON-4-derived
   data), license caveats permitting.

## Repo layout

```
engine/                 Core prediction engine (Rust crate, no UI)
  src/                   Library source
  tests/                 Integration tests
Data/stations/          Per-region harmonic constituent data
docs/                    Architecture, data pipeline, and roadmap notes
```

## Status

This repo currently holds groundwork only: package layout, type shells,
docs, and CI — no working prediction logic yet. See
[`docs/ROADMAP.md`](docs/ROADMAP.md) for the planned build order.

## License

Dual-licensed MIT OR Apache-2.0 (standard for Rust crates meant to be
freely reused as a dependency in other projects — Apache-2.0 adds an
explicit patent grant that MIT alone doesn't have). See
[`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).
Deliberately not depending on any GPL code (e.g. XTide/libtcd); see
`docs/ARCHITECTURE.md` for the reasoning.
