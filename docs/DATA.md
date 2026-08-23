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

Not yet decided. See `docs/ARCHITECTURE.md`'s "Data pipeline" section —
open question is whether to go through TCD (and if so, only as an
offline/build-time conversion step, never at runtime) or source
constituent data directly from each authority in a non-TCD format from
the start. Whatever is chosen, the on-device format should be something
`TidalEngine` can load directly (e.g. JSON or SQLite of `Station` +
`Constituent` records) with no GPL tooling required at runtime.

## Layout

```
Data/stations/bc/   BC station data (empty — first stage, not yet populated)
```

Later stages add sibling directories (`Data/stations/ca/`,
`Data/stations/us/`, etc.) rather than reorganizing what's already
there.
