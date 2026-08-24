# Architecture

## Engine: single Rust core, no GPL dependency

XTide (the well-known open-source harmonic tide engine) is GPL-licensed,
and its `libtcd` reader is part of the same GPL codebase. GPL has a
documented history of tension with app-store distribution models
(confirmed: VLC and GNU Go were both free ($0) apps and still got pulled
from Apple's App Store in 2010-2011 over GPL §6 vs. App Store ToS
restrictions — price does not resolve it).

Decision: **write an original harmonic-prediction implementation**
against the same public-domain formulas XTide itself implements (NOAA
Special Publication No. 98, 1940s, public domain), not derived from
XTide's or any other GPL project's code. This is:

- A well-documented, bounded engineering task, not a research problem —
  the constituent summation is ~20 lines; the harder part (equilibrium
  argument + nodal factor per date) is mechanical astronomical math,
  fully specified in SP 98. Multiple independent open-source
  implementations (e.g. `pytides`) exist for the same reason.
- Dual MIT/Apache-2.0-licensed from the start, so the engine can be
  embedded anywhere — App Store, Play Store, other apps, other people's
  projects — with no copyleft obligation on anyone downstream.

**Language: Rust, not per-platform reimplementation.** Once Android
joined the roadmap, writing (and keeping in sync) an independent copy of
the same math in Swift and Kotlin stopped being the best way to satisfy
"freely reusable as a module in other projects" — a single Rust core is:

- **Genuinely portable**: native compiled code on iOS/Android/desktop/
  server, and can also target WebAssembly for browser/Node use later,
  without rewriting the engine again.
- **One implementation to validate and trust.** The hard-won correctness
  work (constituent summation, equilibrium argument, nodal factors)
  happens once, not twice, and regression tests only need writing once.
- **Bindable, not just embeddable.** [UniFFI](https://mozilla.github.io/uniffi-rs/)
  generates idiomatic Swift and Kotlin bindings directly from the Rust
  crate, so each app calls it as a natural native API, not raw FFI. Now
  wired up (proc-macro style, no `.udl` file): `Station`, `Constituent`,
  `TideExtremum`/`TideExtremumKind`, and `ParseStationError` are UniFFI
  records/enums; `HarmonicPredictor` and `StationLocator` are UniFFI
  objects (`new`/`water_level`/`extrema`/`station` and
  `new`/`nearest` respectively); `parse_station` is an exported
  top-level function. `uniffi::setup_scaffolding!()` in `lib.rs`
  declares the FFI surface from those annotations alone. Bindings are
  generated with `cargo run --features bindgen --bin uniffi-bindgen --
  generate --library <built-lib> --language swift|kotlin --out-dir
  <dir>` — verified in this environment to produce both a working
  `HarmonicPredictor`/`StationLocator`/`Station` Swift API and the
  equivalent Kotlin one (the `bindgen` feature keeps the CLI's `clap`
  dependency out of the iOS/Android `staticlib` build; it's opt-in, not
  default). Actually compiling the generated Swift into an Xcode target
  still needs to happen locally (no Xcode/Swift toolchain here).

Lives at `engine/` (a plain Rust crate — `cargo build` / `cargo test`
work today, no Xcode/Android Studio required, which is why this is the
first thing laid out). `water_level`, `extrema`, and `StationLocator::nearest`
are implemented (`astro.rs`, `nodal.rs`, `species.rs`, `predictor.rs`,
`locator.rs`) and pass self-consistency tests — see `docs/VALIDATION.md`
for exactly what that does and doesn't confirm (real published-prediction
validation is still open, blocked by this environment's network policy).

## Data pipeline: avoid depending on GPL tooling at runtime

Harmonic constituent data is commonly distributed in XTide's TCD binary
format, normally read via `libtcd` (GPL). To keep the shipped app and
engine GPL-free:

- Constituent data is **converted to an open, non-binary format** as an
  offline preprocessing step, not read from TCD at runtime.
- **Decided, for the BC stage**: sidestepped TCD entirely rather than
  choosing a conversion path for it. BC's data was sourced directly —
  real CHS observed water-level series, with harmonic constants fit
  from them (a real harmonic *analysis*, the reverse of prediction —
  see `docs/VALIDATION.md`) — never touching TCD, `libtcd`, or any GPL
  tooling at any point. The `engine::parse_station()` format
  (`engine/src/data.rs`) is a plain `key=value` + `NAME AMPLITUDE PHASE`
  text file, loaded with no external crate dependency and no
  calendar-aware date type (the equilibrium-argument math in
  `engine/src/astro.rs` turned out to need only plain Unix timestamps).
  TCD remains a candidate for *later* stages if a TCD-packaged source
  (e.g. `openwatersio/tide-database`) ends up being the practical path
  there — not decided, not needed for BC.
- Primary candidate source for later stages: `openwatersio/tide-database`
  ("Neaps") — combines NOAA (~3,400 US stations, public domain) and
  TICON-4 (4,200+ global stations, CC BY 4.0), packaged as
  XTide-compatible TCD. For Canada specifically, CHS/DFO's own data (via
  tides.gc.ca / IWLS) was the actual BC-stage source — see `docs/DATA.md`.

See `Data/stations/bc/README.md` for why that directory is empty in
this repo despite BC being the proven first stage — the real data lives
in a private companion repo (CHS licensing/non-commercial use).

## App shells (next step)

Platform order is iOS → Linux → macOS → Android → Windows (see
`docs/ROADMAP.md`), but every shell below is designed with all five in
mind from the start — the point of the Rust engine and the shared
desktop GUI is to not retrofit portability later.

**iOS** (first): SwiftUI + CoreLocation. App target adds `engine/`'s
generated Swift bindings (now wired up — see "Bindable, not just
embeddable" above) as a dependency. Bundles converted station data (see
`Data/`) as app resources. No network calls in the core tide-chart flow.
Not yet scaffolded — needs Xcode locally.

**Desktop — Linux, macOS, Windows** (one shared codebase, built once,
validated on each OS in that order): [`egui`](https://github.com/emilk/egui)
+ `eframe`. Chosen over native-per-OS toolkits (GTK / AppKit / WinUI)
specifically to avoid three separate UI codebases:

- Pure Rust, so it links `engine/` directly as a normal crate
  dependency — no FFI, no generated bindings, unlike the mobile apps.
- MIT/Apache-2.0 licensed, same as `engine/` — no GPL entanglement to
  reason about, unlike e.g. Slint's GPL-licensed edition.
- Ships [`egui_plot`](https://docs.rs/egui_plot) for charting, a direct
  fit for the tidal graph feature.
- **No GPS on desktop.** Unlike iOS/Android, there's no reliable
  cross-platform desktop location API. The desktop MVP therefore needs
  manual location entry as a baseline from day one — what's "change
  location" (post-MVP) on mobile is *required* groundwork on desktop,
  not an enhancement. This can build on whatever
  `StationLocator`/location-selection UI gets built for that feature.

Not yet scaffolded, but — unlike the mobile apps — buildable in an
environment like this one (no Xcode/Android Studio dependency).

**Android** (after the desktop platforms): Kotlin + Jetpack Compose.
App module adds `engine/`'s generated Kotlin (JNI) bindings as a
dependency. Same data-bundling and offline-only approach as iOS. Not
yet scaffolded — needs Android Studio locally.

**Garmin** (after Windows): a real exception to "one Rust engine
everywhere." Connect IQ, Garmin's watch app platform, runs Monkey C —
there's no Rust toolchain targeting it, so `engine/` can't be linked or
bound to it the way the other five platforms do. Two real options, not
yet decided:

1. **Phone computes, watch displays** — the already-built iOS/Android
   app runs `engine/` as normal and pushes computed high/low tide data
   to the paired watch via Connect IQ's companion-app data transfer.
   Standard shape for Garmin apps needing real computation (most
   Garmin tide/weather apps work this way) — no engine duplication, but
   the watch app needs its phone paired to get fresh data.
2. **Monkey C port** — reimplement the harmonic math a third time,
   directly in Monkey C. Works standalone (no phone needed), but means
   a third codebase to keep correct and in sync with `engine/`, working
   against the whole point of the single Rust core.

Defaults to option 1 unless standalone (no-phone) operation turns out to
matter — revisit when this milestone is actually reached.

MVP screen (see `docs/ROADMAP.md` for exact versions): on open, show the
current location (GPS on mobile, manual entry on desktop) and the
current time and water level, then a chart of high/low tide for the
nearest station across a 5-day window (yesterday, today, next 3 days).
Depends on `HarmonicPredictor::extrema` — see `engine/src/predictor.rs`
— for the high/low points, and `HarmonicPredictor::water_level` for the
current reading.

Post-MVP feature order (see `docs/ROADMAP.md`): change location, change
date, configurable day range, full continuous tidal graph (as opposed to
the MVP's high/low-only chart), sunrise/sunset + moon phase — applied
across platforms as each is reached, not iOS-only.

## Sunrise/sunset and moon phase (post-MVP feature)

Both are genuinely related to the tide engine, not just "also
astronomy": moon phase and the spring/neap tide cycle share the same
underlying cause — how aligned the sun and moon are (syzygy at new/full
moon = spring tides; quarter moons, sun and moon at right angles = neap
tides). Practically:

- **Moon phase** ≈ a function of the angular separation between the
  moon's and sun's longitudes — `engine/src/astro.rs` already computes
  both as mean longitudes (`s` and `h`) for the harmonic method, so a
  first-pass phase indicator can reuse them directly rather than adding
  a separate calculation. (Mean longitude is an approximation — the true
  apparent longitude differs by the "equation of center," which can
  shift the exact new/full moon instant by up to roughly a day — fine
  for a phase *indicator*, not for pinpointing the exact moment.)
- **Sunrise/sunset** needs formulas not yet in `astro.rs` (solar
  declination and the hour-angle-based rise/set time, given the
  station's latitude), from the same Meeus/public-domain family as the
  rest of the engine, so it's a natural sibling module, not a new
  approach.

Not implemented yet — this is roadmap groundwork for when that milestone
(`docs/ROADMAP.md` 0.13.0) is reached, not a claim that it works today.

## Garmin (post-Windows platform)

Real exception to "one engine everywhere" — see the "App shells" Garmin
entry above for the two options (phone-computes-and-syncs vs. a Monkey C
port) and why Connect IQ can't just link `engine/` like the other five
targets.

## Open questions carried into the buildout

- **TCD vs. direct-from-authority data sourcing for later stages** —
  resolved for BC (direct, no TCD); still open for Canada-wide/US/global.
- **License check** on TICON-4's underlying GESLA sources before
  bundling. Canada's CHS/DFO data's status is now confirmed as real,
  not just described secondhand: "free under license," non-commercial
  use specifically — see `docs/VALIDATION.md` and the private companion
  repo's `ATTRIBUTION.md`. A proper distribution-safe path for CHS data
  (if one exists) is still open before any Canada-wide bundling that
  isn't personal-use-only.
- **Binding generation** — resolved: UniFFI, proc-macro style, wired up
  and verified to generate working Swift and Kotlin bindings from
  `engine/` (see "Bindable, not just embeddable" above). Desktop doesn't
  need this at all (pure Rust, direct crate dependency).
- **Xcode / Android Studio app-target scaffolding** — needs to happen
  locally, on each respective platform's tooling. The desktop app is the
  one shell buildable in an environment like this one.
- **Desktop location source** — manual entry is the confirmed baseline;
  worth revisiting later whether OS-level location services (Windows
  Location API, macOS Core Location, Linux GeoClue) are worth adding as
  a convenience on top, not a blocker for any milestone.
- **Real-data validation** — `engine/`'s math is implemented and
  self-consistency tested, but not yet checked against any real
  station's published predictions (government tide-data domains are
  blocked by this environment's network policy). See
  `docs/VALIDATION.md` for exactly what's needed and how to unblock it.
- **Garmin phone-sync vs. Monkey C port** — undecided, see above.
