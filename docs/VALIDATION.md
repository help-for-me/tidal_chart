# Validation

What's actually been checked about `engine/`'s correctness, what
hasn't, and the process for closing that gap as data coverage expands
country by country. See [Bibliography](#bibliography) at the bottom for
full source citations.

## What's validated today

**1. Constituent speeds against independently published invariant
constants.** `engine/src/species.rs`'s `known_constituent_speeds_at_j2000`
test computes M2, S2, K1, O1, N2, K2, Q1, and P1's speeds from the
astro/species machinery and checks them against their well-known,
independently published values (M2 = 28.9841042°/hr, S2 = 30.0°/hr
exactly, K1 = 15.0410686°/hr, O1 = 13.9430356°/hr, N2 = 28.4397295°/hr,
K2 = 30.0821373°/hr, Q1 = 13.3986609°/hr, P1 = 14.9589314°/hr). These
are physical constants (derived from the Earth/Moon/Sun's actual orbital
rates), not station-specific data, and don't depend on network access.

**2. Live numeric cross-check against an independent implementation**
(`engine/tests/pytides_cross_check.rs`) — the strongest check in this
repo so far, and the one that actually found and fixed a real bug.
Rather than just reading [pytides](https://github.com/sam-cox/pytides)'
source once and porting formulas by eye (which is all the original
citations amounted to — see below), pytides was **installed and run
live** (Python 3, via the
[`drf5n/pytides` `py3_v2`](https://github.com/drf5n/pytides/tree/py3_v2)
fork — the original doesn't run on Python 3 as-is) against a synthetic
23-constituent station (every species this engine implements, amplitude
1.0/phase 0.0 each — not real station data, just a shared fixture
exercising every nodal-correction formula at once), and its output
compared directly against this engine's, for one week hourly starting
2026-08-24T00:00:00Z:

- **First pass**: max disagreement 0.026, out of a synthetic curve
  ranging roughly ±10. Root cause: pytides deliberately holds nodal
  corrections (u, f) constant across 240-hour blocks as a performance
  optimization (documented in its own source as an approximation, not a
  claim of exactness) — this engine evaluates them at the exact instant
  every time. Reducing pytides' partition size to 0.05 hours (so it
  also evaluates near-instantaneously) dropped the disagreement to
  0.0014 max / 0.0009 mean — confirming the first-pass gap was that
  known approximation difference, not a formula bug.
- **Second pass, with instant-accurate pytides**: a max disagreement of
  0.0014 remained — small, but not the floating-point-noise-level
  agreement expected between two implementations of the same public
  formulas. Investigating traced it to `nodal.rs::f_k2`'s first
  coefficient: this codebase had `0.2523` (from
  [`sam-cox/pytides`](https://github.com/sam-cox/pytides)'s `master`
  branch, `nodal_corrections.py`), but the `drf5n` Python-3 fork has
  `0.2533` instead — **the two "reference" copies of pytides disagree
  with each other**, meaning at least one has an error. Switching this
  engine to `0.2533` dropped the disagreement to **7×10⁻⁶ max, 3×10⁻⁶
  mean** — essentially floating-point-level agreement across all 23
  constituents over a full week. That's about as strong evidence as
  "0.2533 is correct, 0.2523 was wrong" gets without the primary
  Schureman text in hand (see the citation gap below) — **fixed in
  `nodal.rs`**, and the discovery process is preserved in this file and
  in `nodal.rs`'s comment on `f_k2` rather than silently corrected.
- Extrema (`HarmonicPredictor::extrema`) were checked the same way:
  times agree within a few minutes (expected — pytides finds them via
  scipy's Newton's-method solver, this engine via bisection to
  1-second precision; different root-finding methods land on different
  seconds), heights within ~0.005 at those slightly-different times
  (consistent with the curve being locally flat right at an extremum —
  see the comment on `EXTREMA_TIME_WINDOW_SECONDS` in the test file for
  the reasoning).

The golden values in `pytides_cross_check.rs` are a fixed sample from
that live run, so this check now runs automatically in `cargo test`
without needing Python installed — but the *methodology* (install
pytides, run it live, diff the output, investigate any real
disagreement rather than loosening the tolerance until it passes) is
what actually matters here and is worth repeating whenever a new
constituent or formula is added. `engine/examples/cross_check.rs` (`cargo
run --example cross_check -- water_level <start_unix> <hours>` or
`extrema ...`) reproduces this engine's side of that comparison in the
same CSV shape used above, against the same synthetic all-23-species
station, for regenerating golden values against a fresh pytides run.

**3. Internal consistency of the whole pipeline**, via a synthetic
single-constituent (M2-only) station in `engine/src/predictor.rs`'s
tests: extrema alternate High/Low, are spaced ~6.21 hours apart (half
the M2 period), and `water_level()` at each reported extremum time
matches the extremum's own reported height.

## What's cited, and how strong that citation actually is

`astro.rs`/`nodal.rs`/`species.rs` cite Meeus's *Astronomical Algorithms*
(by formula number) and Schureman's NOAA Special Publication No. 98 (by
equation number) — see [Bibliography](#bibliography). **Those citations
were not independently confirmed against the primary texts.** What
actually happened: pytides' source (which itself cites Meeus/Schureman
at those same equation numbers) was fetched and read verbatim from
GitHub, and the formulas/coefficients were ported into original Rust
code, keeping its citations. So the equation numbers here are **pytides'
attribution, carried forward** — and the K2 finding above is direct,
concrete proof that this carries real risk: pytides' own two variants
disagreed with each other on one constant. The live cross-check catches
disagreements *between implementations*; it can't catch a mistake both
implementations happen to share. Only checking against Schureman's
actual text closes that residual gap — see the Bibliography entry for
how to do that once the primary text is available.

## What's NOT validated yet — and why

**No real station's published high/low predictions have been checked
against `engine/`'s output.** This is a different, larger gap than the
pytides cross-check above closes: that check proves this engine
computes the harmonic *method* the way another real implementation
does; it says nothing about whether real government-published harmonic
constants for an actual station, fed through this engine, reproduce
that station's actual published tide times. Only real station data
answers that.

This gap exists because **this environment's network egress proxy
blocks direct access to essentially all general web domains** —
confirmed by testing several very different ones directly, not assumed:

```
gateway answered 403 to CONNECT (policy denial or upstream failure)
  host: www.tides.gc.ca:443            (Canada, CHS/DFO)
  host: api-iwls.dfo-mpo.gc.ca:443     (Canada, CHS/DFO)
  host: api.tidesandcurrents.noaa.gov:443  (US, NOAA)
  host: archive.org:443                 (has the actual Schureman SP-98 scan)
  host: en.wikipedia.org:443
```

It allowlists a narrow set of code-hosting and package-registry domains
(`github.com`, `raw.githubusercontent.com`, `registry.npmjs.org`,
`pypi.org`, `index.crates.io`, a few others — the full list is in the
proxy's own `noProxy` config) and blocks everything else at the policy
level — not specific to government sites, which is how this doc
originally (too narrowly) described it. `WebSearch` still works because
it doesn't route through this local proxy — it returns summarized
results from Anthropic's own search backend, which is how the
archive.org copy of SP-98 and the `pyTMD`/`drf5n` repositories were
*found* even though most of them couldn't be *fetched* directly (GitHub
raw content was the exception — reachable, and how the pytides
cross-check above became possible at all).

Practical effect: this environment can (a) read and run real source
code from GitHub/PyPI, and (b) check output against independently
well-known constants recalled directly. It cannot fetch a primary
document (a government tide-data API, a scanned 1940s book, even
Wikipedia) to check a citation or pull real station data. Both
remaining gaps need the user to fetch and paste in the actual content —
a real BC station's data, or the relevant SP-98 pages — or a future
session with broader network access.

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
   values as the expected output — the same shape as
   `pytides_cross_check.rs`, but against a government source instead of
   another implementation.
5. **Document the source and result** in this file: which station, which
   government API/publication, what date, what the discrepancy was.

This hasn't happened yet for any region — do it before treating any
region's predictions as trustworthy, not just for BC.

**Global stage note**: per `docs/DATA.md`, "global" coverage doesn't
have one authority — it's TICON-4/GESLA-sourced data of mixed licensing
and unclear provenance. When that stage is reached, step 1 above becomes
"survey which countries publish official, checkable tide predictions
and harmonic constants (most maritime nations' hydrographic offices do,
e.g. UKHO, SHOM, BOM), and prioritize validating against those" rather
than assuming one global source suffices.

## Bibliography

**Primary formula sources** (not yet independently confirmed against —
see above; cited here because they're what pytides itself cites, and
what this engine's code comments cite by formula/equation number):

- Meeus, Jean. *Astronomical Algorithms*, 2nd ed. Willmann-Bell, 1998.
  Formulas cited in `engine/src/astro.rs`: 7.1 (Julian Day), 11.1
  (Julian centuries), 21.3 (mean obliquity), 24.2 (solar longitude),
  45.1 (lunar longitude), 45.7 (lunar node).
- Schureman, Paul. *Manual of Harmonic Analysis and Prediction of
  Tides.* U.S. Coast and Geodetic Survey Special Publication No. 98,
  1940 (revised, reprinted with corrections 1958). Public domain (U.S.
  government work). Equation numbers cited in `engine/src/nodal.rs`:
  65, 66, 67, 68, 69, 70, 71, 73, 74, 75, 76, 77, 78, 149, 195, 202, 204,
  206, 207, 213, 214, 215, 224, 226, 227, 232, 234, 235; Tables 2 and 6.
  A public-domain scan exists at
  [archive.org](https://archive.org/details/manualofharmonic00schu)
  (found via search; not fetchable from this environment — see above).
  A PDF is also referenced at NOAA's own
  [tidesandcurrents.noaa.gov/publications/SpecialPubNo98.pdf](https://tidesandcurrents.noaa.gov/publications/SpecialPubNo98.pdf)
  (same access constraint).

**Reference implementation, consulted and run live** (see "What's
validated today" above for exactly how each was used):

- Cox, Sam. `pytides`. MIT License.
  [github.com/sam-cox/pytides](https://github.com/sam-cox/pytides) —
  source read verbatim and ported from (not copied — original Rust code
  written from the same Meeus/Schureman formulas pytides also
  implements); its `nodal_corrections.py` has `f_K2`'s first coefficient
  as `0.2523`, which this engine inherited and later found to disagree
  with the fork below.
- `drf5n`. `pytides`, `py3_v2` branch (Python 3 port of the above).
  [github.com/drf5n/pytides/tree/py3_v2](https://github.com/drf5n/pytides/tree/py3_v2)
  — installed and run live for the cross-check in
  `engine/tests/pytides_cross_check.rs`. Its `f_K2` has `0.2533` instead
  of `0.2523` — this engine now matches this value, based on the
  cross-check evidence above, though neither pytides variant's value has
  been confirmed against Schureman's actual text (see the citation gap
  above).

**Not yet consulted, but identified as candidates** (found via
`WebSearch`, not fetched or read):

- `pyTMD` (Sutterley et al.), an actively maintained tidal-prediction
  package. [github.com/tsutterley/pyTMD](https://github.com/tsutterley/pyTMD).
  Its `constituents.py::nodal_modulation` uses a structurally different
  nodal-correction approach (a linearized small-angle approximation,
  the "Ray 1999" style used by OTIS/FES-family software) rather than
  Schureman's exact spherical-trigonometry formulas this engine and
  pytides both implement — not directly comparable term-by-term, so it
  wasn't useful as a tie-breaker for the K2 discrepancy above, but worth
  a look as a second cross-check once more constituents/compound terms
  are added.
