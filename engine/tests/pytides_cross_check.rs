//! Cross-check against [pytides](https://github.com/sam-cox/pytides)
//! (MIT-licensed), an independent Python implementation of the same
//! harmonic method — run live and compared numerically, not just read
//! for reference. See `docs/VALIDATION.md` for the full methodology,
//! including how this and a direct check against Schureman's actual
//! 1958 text together found and fixed two real issues: `f_k1`/`f_k2`
//! stayed correct throughout, but `f_m1` had a genuine wrong-exponent
//! bug shared with both pytides variants (so this comparison alone
//! couldn't have caught it — only the primary text could, and did —
//! see `nodal.rs::f_m1`'s comment), and an earlier attempt to "fix"
//! `f_k2` based on disagreement with a buggy pytides fork was itself
//! wrong and has been reverted (see `nodal.rs::f_k2`'s comment).
//!
//! Golden values below are pytides' output (the `drf5n/pytides` `py3_v2`
//! Python 3 fork, patched to match Schureman's actual text for `f_k2`
//! and `f_m1` — see `docs/VALIDATION.md` — with its `at()`/`extrema()`
//! partition size reduced from the default 240 hours to 0.05 hours so
//! nodal corrections are evaluated at each instant rather than held
//! constant across a multi-day block — that partitioning is a
//! deliberate, documented pytides performance optimization, not a claim
//! of exactness, so comparing against it at pytides' default coarseness
//! would be comparing against a different, less precise computation
//! than this engine's, not a fair correctness check) for a synthetic
//! station using all 23 constituent species this engine implements,
//! each at amplitude 1.0 / phase 0.0 — not real station data, just a
//! shared fixture exercising every species' nodal corrections at once.
//! Full run: 2026-08-24T00:00:00Z, one week hourly, all 23 species; max
//! absolute water_level disagreement 7e-6, mean 3e-6. The tolerance
//! below leaves an order of magnitude of margin over that.

use tidal_engine::{Constituent, HarmonicPredictor, Station, TideExtremumKind};

const NAMES: &[&str] = &[
    "Z0", "Sa", "Ssa", "Mm", "Mf", "Q1", "O1", "K1", "J1", "M1", "P1", "S1", "OO1", "2N2", "N2",
    "nu2", "M2", "lambda2", "L2", "T2", "S2", "R2", "K2", "M3",
];

fn synthetic_station() -> Station {
    let constituents = NAMES
        .iter()
        .map(|name| Constituent::new(*name, 1.0, 0.0))
        .collect();
    Station::new(
        "synthetic-all",
        "Synthetic all-species station",
        49.28,
        -123.12,
        0.0,
        constituents,
    )
}

const TOLERANCE: f64 = 1e-4;

#[test]
fn water_level_matches_pytides() {
    let predictor = HarmonicPredictor::new(synthetic_station());

    // (unix_time_seconds, pytides water_level)
    let golden: &[(i64, f64)] = &[
        (1787529600, -0.343260825),
        (1787569200, 4.439108292),
        (1787608800, 2.857281716),
        (1787648400, 0.9625407123),
        (1787688000, -1.722253267),
        (1787727600, -1.673756079),
        (1787767200, -2.428247694),
        (1787806800, -3.360433309),
        (1787846400, 3.106386465),
        (1787886000, -2.551909811),
        (1787925600, 5.174321078),
        (1787965200, 1.033068722),
        (1788004800, 6.93790877),
        (1788044400, 2.89802971),
        (1788084000, 7.515079291),
    ];

    for (unix_time, expected) in golden {
        let got = predictor.water_level(*unix_time);
        assert!(
            (got - expected).abs() < TOLERANCE,
            "at {unix_time}: got {got}, pytides says {expected} (diff {})",
            (got - expected).abs()
        );
    }
}

/// pytides finds extrema via scipy's `fsolve` (Newton's method on an
/// analytic first/second derivative pair); this engine bisects the
/// first derivative to 1-second precision (see `predictor.rs`) — a
/// different root-finding method, so the two don't land on exactly the
/// same second. Observed agreement is within a few minutes. Right at an
/// extremum the curve is locally flat (derivative is zero there by
/// definition), so a several-minute timing difference should only
/// produce a small height difference, not a proportionally large one —
/// so extrema get a looser time-matching window and height tolerance
/// than the same-instant `water_level_matches_pytides` check above,
/// which is the stricter, primary comparison.
const EXTREMA_TIME_WINDOW_SECONDS: i64 = 900;
const EXTREMA_HEIGHT_TOLERANCE: f64 = 0.01;

#[test]
fn extrema_match_pytides() {
    let predictor = HarmonicPredictor::new(synthetic_station());

    // (unix_time_seconds, pytides height, kind) - a sample spanning the
    // same week as the water_level golden values above.
    let golden: &[(i64, f64, TideExtremumKind)] = &[
        (1787545125, -8.0518928410, TideExtremumKind::Low),
        (1787573280, 5.2954200548, TideExtremumKind::High),
        (1787716937, -5.4156880400, TideExtremumKind::Low),
        (1787913995, 7.5587163436, TideExtremumKind::High),
        (1788086558, 7.9032692787, TideExtremumKind::High),
        (1788108363, -1.1756782614, TideExtremumKind::Low),
    ];

    let extrema = predictor.extrema(1_787_529_600, 1_787_529_600 + 7 * 86_400);

    for (unix_time, expected_height, expected_kind) in golden {
        let found = extrema
            .iter()
            .find(|e| (e.unix_time_seconds - unix_time).abs() <= EXTREMA_TIME_WINDOW_SECONDS)
            .unwrap_or_else(|| panic!("no extremum found near pytides' t={unix_time}"));
        assert_eq!(found.kind, *expected_kind, "at {unix_time}: wrong High/Low");
        assert!(
            (found.height - expected_height).abs() < EXTREMA_HEIGHT_TOLERANCE,
            "at {unix_time}: got height {}, pytides says {expected_height}",
            found.height
        );
    }
}
