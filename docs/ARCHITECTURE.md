# Architecture

## Engine: original implementation, no GPL dependency

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
- MIT-licensed from the start, so the engine can be embedded anywhere —
  App Store, other apps, other people's projects — with no copyleft
  obligation on anyone downstream.

## Data pipeline: avoid depending on GPL tooling at runtime

Harmonic constituent data is commonly distributed in XTide's TCD binary
format, normally read via `libtcd` (GPL). To keep the shipped app and
engine GPL-free:

- Constituent data should be **converted to an open, non-binary format**
  (e.g. JSON or SQLite) as an offline preprocessing step, not read from
  TCD at runtime.
- That conversion step may use existing GPL tooling (e.g. `libtcd`,
  XTide utilities) as a *build-time-only* tool against the data — GPL
  covers software, not the underlying empirical harmonic constants
  themselves, which come from NOAA/CHS/etc. publications. **This still
  needs a real decision** before the data pipeline is built: whether to
  go through TCD at all, or source constituent data directly from each
  authority's own non-TCD publications (e.g. NOAA's API, CHS/IWLS) to
  sidestep the question entirely.
- Primary candidate source: `openwatersio/tide-database` ("Neaps") —
  combines NOAA (~3,400 US stations, public domain) and TICON-4
  (4,200+ global stations, CC BY 4.0), packaged as XTide-compatible TCD.
  For Canada specifically, CHS/DFO's own data (via tides.gc.ca / IWLS)
  may be a cleaner direct source — see `docs/DATA.md`.

See `Data/stations/bc/README.md` for the concrete first-region pipeline.

## App shell (next step)

Not yet scaffolded in this repo. The `TidalEngine` package here is
platform-agnostic (iOS 17+ / macOS 14+) and testable with `swift test`
without Xcode, which is why it's the first thing laid out. The SwiftUI
app itself (CoreLocation integration, tide chart UI, App target,
entitlements, Info.plist) needs to be created as an Xcode project that
depends on this package — that has to happen in Xcode locally, not in
this environment. Planned shape, per the original idea capture:

- SwiftUI + CoreLocation for GPS.
- App target adds `TidalEngine` as a local Swift package dependency.
- Bundles converted station data (see `Data/`) as app resources.
- No network calls in the core tide-chart flow.

## Open questions carried into the buildout

- **TCD vs. direct-from-authority data sourcing** — see above, affects
  the whole data pipeline design.
- **License check** on TICON-4's underlying GESLA sources and on
  Canada's CHS/DFO data (described as "free under license," not flatly
  public domain like NOAA) before bundling either.
- **Xcode app-target scaffolding** — needs to happen locally.
