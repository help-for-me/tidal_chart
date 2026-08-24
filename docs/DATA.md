# Data plan

## Staged regional rollout

1. **BC** (British Columbia) — first target. Small enough to validate
   the full pipeline (source → convert → bundle → predict) end to end
   before scaling up.
2. **Canada** — expand to full CHS/DFO coverage once the BC pipeline is
   proven.
3. **Canada + US** — add NOAA's ~3,400 stations (public domain,
   auto-updated monthly via NOAA's API).
4. **Global** — broaden further (e.g. TICON-4-derived, 4,200+ stations),
   license caveats permitting (see below).

Each stage is a data-only expansion — the engine and app shouldn't need
structural changes to go from one stage to the next, just more `Station`
records loaded.

## Candidate sources

- **CHS/DFO** (Canada) — via tides.gc.ca / the IWLS web service. Primary
  candidate for the BC stage. Licensing note from the original idea
  capture: described as "free under license," not flatly public domain
  like NOAA's data — needs an actual license check before bundling, not
  assumed clear.
- **NOAA** — ~3,400 US stations, public domain, relevant from the
  "Canada + US" stage onward.
- **openwatersio/tide-database ("Neaps")** — a maintained dataset
  combining NOAA + TICON-4 (4,200+ global stations, CC BY 4.0), packaged
  as XTide-compatible TCD files. Useful as a reference/cross-check even
  if BC's own CHS data is sourced directly.
- **TICON-4** — relevant at the "Global" stage. Underlying GESLA-sourced
  licenses described as "mixed, not fully spelled out" in the original
  idea capture — needs a look before bundling.

## Format

**Decided, for the BC stage**: a simple, dependency-free `key=value` +
`NAME AMPLITUDE PHASE` text format, read by `engine::parse_station()`
(`engine/src/data.rs`) — no JSON/SQLite, no TCD, no GPL tooling
anywhere in the loading path. The TCD-vs-direct-sourcing question is
moot for this stage: BC's data was sourced directly (real CHS observed
water levels, harmonic constants fit from them — see
`docs/VALIDATION.md`), never touching TCD at all. Worth revisiting for
later stages if a TCD-packaged source (e.g. openwatersio/tide-database)
ends up being the practical path there.

## Layout

```
Data/stations/bc/   Empty in this public repo, by design — see below.
```

**Real BC station data isn't in this repo.** It's derived from CHS
data that isn't flatly public domain and is for personal use only (see
`docs/VALIDATION.md`'s Bibliography and the private companion repo's
`ATTRIBUTION.md`) — committing it here, under this repo's permissive
MIT/Apache license, would make it freely redistributable, including
commercially, which contradicts that. Four BC stations are bundled in
`engine::parse_station()`'s format in the private
`tidal_chart-station-data` repo instead. Later stages add sibling
directories (`Data/stations/ca/`, `Data/stations/us/`, etc.) to this
same empty structure once a distribution-safe source is found for each
— see `docs/ROADMAP.md`.
