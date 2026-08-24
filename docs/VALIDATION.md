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

**Sources used**: the astronomical-element formulas (Meeus's
*Astronomical Algorithms*, as used for Schureman's equations in NOAA
Special Publication No. 98, "Manual of Harmonic Analysis and Prediction
of Tides" — both public domain) were implemented from scratch, with
[pytides](https://github.com/sam-cox/pytides) (MIT-licensed) consulted
as a working cross-reference for the exact numeric coefficients and
Doodson/XDO decoding — the same practice already documented in
`docs/ARCHITECTURE.md`'s "not derived from XTide's or pytides' own
code" principle: the *code* here is original, the constants are the
same published astronomical facts pytides also implements.

## What's NOT validated yet — and why

**No real station's published high/low predictions have been checked
against `engine/`'s output.** The regression tests above confirm the
*machinery* is self-consistent and matches known physical constants,
but that's not the same as confirming a real CHS or NOAA station's
actual published tide times match what this engine predicts from that
station's real harmonic constants.

This gap exists because **this development environment's network egress
proxy blocks government tide-data domains at the policy level** —
confirmed directly, not assumed:

```
gateway answered 403 to CONNECT (policy denial or upstream failure)
  host: www.tides.gc.ca:443
  host: api-iwls.dfo-mpo.gc.ca:443
  host: www.dfo-mpo.gc.ca:443
  host: api.tidesandcurrents.noaa.gov:443
  host: tidesandcurrents.noaa.gov:443
```

(github.com/raw.githubusercontent.com, used above for the pytides
cross-reference, is not blocked — this is specific to government
domains, not a general network outage.)

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
