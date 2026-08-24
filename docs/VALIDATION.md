# Validation

What's actually been checked about `engine/`'s correctness, what
hasn't, and the process for closing that gap as data coverage expands
country by country.

## What's validated today

**Constituent speeds against independently published invariant
constants.** `engine/src/species.rs`'s `known_constituent_speeds_at_j2000`
test computes M2, S2, K1, O1, N2, K2, Q1, and P1's speeds from the
astro/species machinery and checks them against their well-known,
independently published values (M2 = 28.9841042°/hr, S2 = 30.0°/hr
exactly, K1 = 15.0410686°/hr, O1 = 13.9430356°/hr, N2 = 28.4397295°/hr,
K2 = 30.0821373°/hr, Q1 = 13.3986609°/hr, P1 = 14.9589314°/hr). These
are physical constants (derived from the Earth/Moon/Sun's actual orbital
rates), not station-specific data, and don't depend on network access —
they're a strong check on the astronomical-element machinery (`astro.rs`)
and the Doodson-coefficient decoding (`species.rs`), independent of any
real station's harmonic constants.

**Internal consistency of the whole pipeline**, via a synthetic
single-constituent (M2-only) station in `engine/src/predictor.rs`'s
tests: extrema alternate High/Low, are spaced ~6.21 hours apart (half
the M2 period, 360°/28.9841042° per hour), and `water_level()` at each
reported extremum time matches the extremum's own reported height. This
exercises astro → species lookup → nodal corrections (u, f) → the
derivative-based bisection search end to end, without needing real
station data.

**Sources cited in code comments, and an honest note on how strong that
citation actually is**: `astro.rs`/`nodal.rs`/`species.rs` cite Meeus's
*Astronomical Algorithms* (by formula number: 7.1, 11.1, 21.3, 24.2,
45.1, 45.7) and Schureman's NOAA Special Publication No. 98, "Manual of
Harmonic Analysis and Prediction of Tides" (by equation number, e.g. 73,
65, 227, 226, 214, 202 — both public domain). **Those citations were not
independently confirmed against the primary texts** — this environment
can't fetch either book directly (see below). What actually happened:
[pytides](https://github.com/sam-cox/pytides) (MIT-licensed) — an
existing, real implementation that itself cites Meeus/Schureman at those
same equation numbers — was fetched and read verbatim (its source is on
GitHub, which *is* reachable here), and the formulas/coefficients were
ported from it into original Rust code, keeping its citations. So the
equation numbers in this codebase are **pytides' attribution, carried
forward, not independently verified against Schureman's or Meeus's
actual text.** If pytides mis-cited or mis-transcribed one formula, this
codebase would have inherited that silently — nothing in the current
tests would catch a single wrong nodal-correction formula the way the
speed-constant test would catch a wrong spanning-set coefficient.

The one thing that *is* independent of pytides being right about
anything: the constituent speeds above are widely, separately
republished physical constants (not something only pytides asserts),
and they match exactly.

## What's NOT validated yet — and why

**No real station's published high/low predictions have been checked
against `engine/`'s output**, and **the Meeus/Schureman citations
haven't been checked against the primary texts themselves** (see above).
Both gaps trace back to the same underlying constraint, and it's broader
than "government tide-data sites are blocked" — that was this doc's
first (too narrow) explanation; the real shape of it, confirmed by
testing several different domains directly:

```
gateway answered 403 to CONNECT (policy denial or upstream failure)
  host: www.tides.gc.ca:443            (Canada, CHS/DFO)
  host: api-iwls.dfo-mpo.gc.ca:443     (Canada, CHS/DFO)
  host: api.tidesandcurrents.noaa.gov:443  (US, NOAA)
  host: archive.org:443                 (has the actual Schureman SP-98 scan)
  host: en.wikipedia.org:443
```

This environment's network egress proxy allowlists a narrow set of
code-hosting and package-registry domains (`github.com`,
`raw.githubusercontent.com`, `registry.npmjs.org`, `pypi.org`,
`index.crates.io`, a few others — the full list is in the proxy's own
`noProxy` config) and blocks essentially everything else at the policy
level, government sites included but not specific to them. `WebSearch`
still works because it doesn't route through this local proxy at all —
it returns summarized results from Anthropic's own search backend, which
is how the archive.org copy of SP-98 was *found* even though it can't be
*fetched* to actually read.

Practical effect: this environment can verify things by (a) reading real
source code from GitHub, and (b) checking output against independently
well-known constants recalled directly. It cannot fetch a primary
document (a government tide-data API, a scanned 1940s public-domain
book, even Wikipedia) to check a citation or pull real station data
directly. Both remaining gaps need the user to fetch and paste in the
actual content — either a real BC station's data (for the prediction
validation), or the relevant pages of SP-98 (for the citation check) —
or a future environment/session with broader network access.

## The validation process (bake this in per region)

Each data-coverage stage in `docs/ROADMAP.md` (BC → Canada → Canada+US
→ Global) needs a **real-data validation pass** before it's actually
trustworthy, not just "implemented":

1. **Identify the authoritative government source** for that region
   (Canada: CHS/DFO via tides.gc.ca / the IWLS API; US: NOAA CO-OPS via
   api.tidesandcurrents.noaa.gov — both documented in `docs/DATA.md`).
2. **Fetch one real station's harmonic constants** from that source
   (amplitude + phase per constituent) and its **published high/low
   tide predictions** for some date range.
3. **Run that station through `HarmonicPredictor`** and compare its
   predicted extrema times/heights against the government-published
   ones. Record the actual discrepancy (time offset, height error) —
   don't just check "close enough" by eye.
4. **Add it as a real regression test** (`engine/tests/`) once it
   passes, using that station's real constants and real published
   values as the expected output — the same shape as the synthetic M2
   test in `predictor.rs`, but with real data.
5. **Document the source and result** in this file: which station, which
   government API/publication, what date, what the discrepancy was.

This hasn't happened yet for any region. It needs an environment (or a
manual step where the user fetches the data) with access to the
relevant government domain — do this before treating any region's
predictions as trustworthy, not just for BC.

**Global stage note**: per `docs/DATA.md`, "global" coverage doesn't
have one authority — it's TICON-4/GESLA-sourced data of mixed licensing
and unclear provenance. When that stage is reached, step 1 above becomes
"survey which countries publish official, checkable tide predictions
and harmonic constants (most maritime nations' hydrographic offices do,
e.g. UKHO, SHOM, BOM), and prioritize validating against those" rather
than assuming one global source suffices.

## Separately: checking the Meeus/Schureman citations themselves

Lower priority than real-station validation (the code's *output* is
already checked against independent constants; this would only confirm
the *documentation* is honest about which equation is which), but worth
doing before calling the citations trustworthy:

1. Schureman's SP-98 has a public-domain scan at
   [archive.org](https://archive.org/details/manualofharmonic00schu)
   (found via search, not fetchable directly from this environment —
   see above). The equation numbers cited in `nodal.rs` (73, 65, 74, 66,
   75, 67, 76, 68, 77, 69, 78, 70, 227, 226, 215, 213, 204, 235, 234, 71,
   206, 207, 195, 149) and Tables 2/6 are the ones to check.
2. Paste the relevant pages' text in, or download the PDF and hand it
   over directly — either works.
3. Compare each cited equation's actual formula against what's in
   `nodal.rs`; record any discrepancy found (and fix it) here.
