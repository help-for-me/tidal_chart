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

**2. Six formulas checked directly against Schureman's actual 1958
text** — `f_k1`, `u_k1` (via `nup`), `f_k2`, `u_k2` (via `nupp`), `f_l2`,
`u_l2`, and `u_m1` (equations 195, 202, 213, 214, 223, 224, 227, 232,
234, 235) — all confirmed to match exactly. **One, `f_m1`, was found
wrong and fixed**: both pytides variants (and this engine, which had
ported from them) had `cos(0.5*I)` raised to the `-0.5` power in one
term; Schureman's actual equation 195 requires `-2`, not `-0.5` — a real
bug, not a citation-strength technicality, and one the live pytides
cross-check below could never have caught, because pytides has the same
bug (see `nodal.rs::f_m1`'s comment for the full detail).

How this happened: `docs/VALIDATION.md`'s original version said checking
the primary text wasn't possible in this environment (the government/
archive.org domains that host it are blocked — see below). That was
true for *those* copies, but a scanned copy of Schureman 1958 turned out
to be bundled directly in a GitHub repository
([`JacksonKearl/solunar`](https://github.com/JacksonKearl/solunar)),
reachable the same way pytides' source was. `poppler-utils` and
`tesseract` were installed via `apt` to work with it; OCR across all 336
pages turned out to be impractically slow in this environment, so
specific pages were instead viewed directly (image rendering + reading
the equations off the page) once the right page range was located by
sampling.

**This also caught a mistake made earlier in this same session**: an
initial cross-check disagreement in `f_k2` was "fixed" by changing this
engine's `0.2523` to `0.2533`, matching the `drf5n` Python-3 pytides
fork more closely. That was backwards — Schureman's actual equation 235
confirms `0.2523` is correct; the `drf5n` fork has the error, not
`sam-cox/pytides`'s original. **Reverted.** The lesson generalizes:
agreement between two implementations proves they compute the same
thing, not that the thing they compute is right — only the primary text
settles that, and in this specific case, settled it in the opposite
direction from what the cross-check evidence alone suggested.

**3. Live numeric cross-check against an independent implementation**
(`engine/tests/pytides_cross_check.rs`) — installed and ran
[pytides](https://github.com/sam-cox/pytides) live (Python 3, via the
[`drf5n/pytides` `py3_v2`](https://github.com/drf5n/pytides/tree/py3_v2)
fork — the original doesn't run on Python 3 as-is), rather than just
reading its source once and porting formulas by eye, against a synthetic
23-constituent station (every species this engine implements, amplitude
1.0/phase 0.0 each — not real station data, just a shared fixture
exercising every nodal-correction formula at once), for one week hourly
starting 2026-08-24T00:00:00Z:

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
  0.0014 remained, traced (incorrectly, see above) to `f_k2`.
- **After the primary-text check above** (`f_k2` reverted to `0.2523`,
  `f_m1` fixed to the `-2` exponent, and the local pytides copy patched
  to match on both so it's comparing two now-correct implementations):
  **7×10⁻⁶ max, 3×10⁻⁶ mean** disagreement — essentially
  floating-point-level agreement across all 23 constituents over a full
  week.
- Extrema (`HarmonicPredictor::extrema`) were checked the same way:
  times agree within a few minutes (expected — pytides finds them via
  scipy's Newton's-method solver, this engine via bisection to
  1-second precision; different root-finding methods land on different
  seconds), heights within ~0.01 at those slightly-different times
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

**4. Internal consistency of the whole pipeline**, via a synthetic
single-constituent (M2-only) station in `engine/src/predictor.rs`'s
tests: extrema alternate High/Low, are spaced ~6.21 hours apart (half
the M2 period), and `water_level()` at each reported extremum time
matches the extremum's own reported height.

## What's cited, and how strong that citation actually is — updated

`astro.rs`/`nodal.rs`/`species.rs` cite Meeus's *Astronomical Algorithms*
(by formula number) and Schureman's NOAA Special Publication No. 98 (by
equation number) — see [Bibliography](#bibliography). **Seven formulas
(the K1/K2/L2 pair and `u_m1`) are now confirmed directly against
Schureman's primary text; `f_m1` was checked and found wrong, then
fixed; everything else in `nodal.rs` and all of `astro.rs`'s Meeus
citations are still only pytides' attribution, carried forward when its
source was ported, not independently confirmed.** The remaining
unchecked formulas (`f_mm`, `f_mf`, `f_o1`/`u_o1`, `f_j1`/`u_j1`,
`f_oo1`/`u_oo1`, `f_m2`/`u_m2`, `f_modd`/`u_modd`, and all of `astro.rs`)
are simpler, single-term expressions than the K1/K2/L2/M1 group (which
are special specifically because each combines two nearly-equal-speed
terms — exactly the kind of derivation that produced the `f_m1` bug),
so they're lower-risk, but "lower-risk" isn't "confirmed." The live
pytides cross-check catches disagreements *between implementations*; it
provably cannot catch a mistake both implementations share, which is
exactly what happened with `f_m1` — only the primary text caught that
one. Checking the remaining formulas the same way (locate the page,
read the equation, compare) is the way to close the rest of this gap.

## What's NOT validated yet — and why

**No real station's published high/low predictions have been checked
against `engine/`'s output.** This is a different, larger gap than the
formula-level checks above close: those checks prove this engine
computes the harmonic *method* correctly; they say nothing about whether
real government-published harmonic constants for an actual station, fed
through this engine, reproduce that station's actual published tide
times. Only real station data answers that.

This gap exists because **this environment's network egress proxy
blocks direct access to most general web domains** — confirmed by
testing several very different ones directly, not assumed:

```
gateway answered 403 to CONNECT (policy denial or upstream failure)
  host: www.tides.gc.ca:443            (Canada, CHS/DFO)
  host: api-iwls.dfo-mpo.gc.ca:443     (Canada, CHS/DFO)
  host: api.tidesandcurrents.noaa.gov:443  (US, NOAA)
  host: archive.org:443
  host: en.wikipedia.org:443
```

It allowlists a narrow set of code-hosting and package-registry domains
(`github.com`, `raw.githubusercontent.com`, `registry.npmjs.org`,
`pypi.org`, `index.crates.io`, a few others — the full list is in the
proxy's own `noProxy` config) and blocks everything else at the policy
level — not specific to government sites. **This is why the primary
Schureman text was reachable at all**: not from archive.org or NOAA's
own PDF (both blocked, as shown above), but because someone had
committed a scanned copy directly into a GitHub repository, and GitHub
is allowlisted. `WebSearch` also works because it doesn't route through
this local proxy — it returns summarized results from Anthropic's own
search backend, which is how that repository and `pyTMD` were *found*.

Practical effect: this environment can read and run real source code
and data from GitHub/PyPI (which turned out to include a primary
19th/20th-century government document, once one was found hosted
there), and can check output against independently well-known constants
recalled directly. It generally cannot fetch content from most other
web domains — government APIs for real station data being the
remaining, and larger, gap. That needs the user to fetch and paste in
the actual content — a real BC station's data — or a future session
with broader network access.

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

**Primary formula sources:**

- Meeus, Jean. *Astronomical Algorithms*, 2nd ed. Willmann-Bell, 1998.
  Formulas cited in `engine/src/astro.rs`: 7.1 (Julian Day), 11.1
  (Julian centuries), 21.3 (mean obliquity), 24.2 (solar longitude),
  45.1 (lunar longitude), 45.7 (lunar node). **Not yet independently
  confirmed against this primary text** — unlike Schureman below, no
  accessible copy has been located yet.
- Schureman, Paul. *Manual of Harmonic Analysis and Prediction of
  Tides.* U.S. Coast and Geodetic Survey Special Publication No. 98,
  1940 (revised, reprinted with corrections 1958). Public domain (U.S.
  government work). Equation numbers cited in `engine/src/nodal.rs`:
  65, 66, 67, 68, 69, 70, 71, 73, 74, 75, 76, 77, 78, 149, 195, 202, 204,
  206, 207, 213, 214, 215, 224, 226, 227, 232, 234, 235; Tables 2 and 6.
  **Directly consulted**: a scanned copy (336 pages) was obtained from
  [`github.com/JacksonKearl/solunar`](https://github.com/JacksonKearl/solunar)'s
  `Schureman1958.pdf` (raw.githubusercontent.com, not archive.org or
  NOAA's own copy — both blocked, see above); equations 195, 202, 210-
  235 (pp. 41-48, covering L2/M1/K1/K2) were read directly and compared
  against `nodal.rs` — see "What's validated today" above for what
  matched and what didn't. Equations 65-78 (Mm/Mf/O1/J1/OO1/M2) were
  not located in this pass — they don't appear to get Schureman's own
  dedicated derivation the way K1/K2/L2/M1 do (those four are singled
  out specifically because each combines two nearly-equal-speed terms),
  so the citation numbers pytides carries for them are unconfirmed
  guesses at this book's actual structure, not verified page references.
  The same repository also has `Zetler1982.pdf` ("Extensions of Tidal
  Prediction Tables to Include Shallow Water Constituents"), unused so
  far but a candidate source once compound/shallow-water constituents
  are added.

**Reference implementations, consulted and run live:**

- Cox, Sam. `pytides`. MIT License.
  [github.com/sam-cox/pytides](https://github.com/sam-cox/pytides) —
  source read verbatim and ported from (not copied — original Rust code
  written from the same Meeus/Schureman formulas pytides also
  implements). Its `f_K2` (`0.2523`) and `f_M1` (the buggy `-0.5`
  exponent) match Schureman's actual text and this engine's code exactly
  for the former; both it and the fork below share the latter's bug.
- `drf5n`. `pytides`, `py3_v2` branch (Python 3 port of the above).
  [github.com/drf5n/pytides/tree/py3_v2](https://github.com/drf5n/pytides/tree/py3_v2)
  — installed and run live for the cross-check in
  `engine/tests/pytides_cross_check.rs`. Its `f_K2` (`0.2533`) is a
  transcription error not present in `sam-cox/pytides`'s original —
  confirmed by direct comparison against Schureman's text above, after
  this engine briefly and incorrectly adopted the same error mid-session
  (see "What's validated today").

**Surveyed but not used:**

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
