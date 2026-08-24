//! The known tidal constituent *species* — the fixed astronomical
//! definition of each named constituent (M2, S2, K1, O1, ...): its
//! equilibrium-argument coefficients against the spanning set
//! `[T+h-s, s, h, p, N, pp, 90]` (Doodson/XDO convention), and which
//! nodal-correction formulas (`crate::nodal`) apply to it.
//!
//! This is distinct from `crate::Constituent`, which holds a *station's*
//! empirical amplitude/phase for a named species.
//!
//! Coefficients were decoded from the standard XDO letter notation
//! (A=1, B=2, ..., Z=0, R=-8 ... Y=-1) cross-checked against pytides'
//! `constituent.py` (MIT-licensed) as a working reference, then
//! verified independently: the resulting dot-product speeds for M2,
//! S2, K1, O1, N2, K2, Q1, and P1 match their well-known, independently
//! published values exactly — see the regression test below. Covers
//! the base (non-compound) constituents; compound/shallow-water
//! constituents (M4, MS4, etc.) are a documented follow-up, not
//! included yet.

use crate::astro::Astro;
use crate::nodal as nc;

pub(crate) struct Species {
    pub name: &'static str,
    /// Coefficients against `[T+h-s, s, h, p, N, pp, 90]`.
    pub coefficients: [f64; 7],
    pub u: fn(&Astro) -> f64,
    pub f: fn(&Astro) -> f64,
}

#[rustfmt::skip]
pub(crate) const SPECIES: &[Species] = &[
    // Long-term
    Species { name: "Z0",  coefficients: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "Sa",  coefficients: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "Ssa", coefficients: [0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "Mm",  coefficients: [0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0], u: nc::u_zero, f: nc::f_mm },
    Species { name: "Mf",  coefficients: [0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_mf, f: nc::f_mf },
    // Diurnals
    Species { name: "Q1",  coefficients: [1.0, -2.0, 0.0, 1.0, 0.0, 0.0, 1.0], u: nc::u_o1, f: nc::f_o1 },
    Species { name: "O1",  coefficients: [1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0], u: nc::u_o1, f: nc::f_o1 },
    Species { name: "K1",  coefficients: [1.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0], u: nc::u_k1, f: nc::f_k1 },
    Species { name: "J1",  coefficients: [1.0, 2.0, 0.0, -1.0, 0.0, 0.0, -1.0], u: nc::u_j1, f: nc::f_j1 },
    Species { name: "M1",  coefficients: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0], u: nc::u_m1, f: nc::f_m1 },
    Species { name: "P1",  coefficients: [1.0, 1.0, -2.0, 0.0, 0.0, 0.0, 1.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "S1",  coefficients: [1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "OO1", coefficients: [1.0, 3.0, 0.0, 0.0, 0.0, 0.0, -1.0], u: nc::u_oo1, f: nc::f_oo1 },
    // Semi-diurnals
    Species { name: "2N2",     coefficients: [2.0, -2.0, 0.0, 2.0, 0.0, 0.0, 0.0], u: nc::u_m2, f: nc::f_m2 },
    Species { name: "N2",      coefficients: [2.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0], u: nc::u_m2, f: nc::f_m2 },
    Species { name: "nu2",     coefficients: [2.0, -1.0, 2.0, -1.0, 0.0, 0.0, 0.0], u: nc::u_m2, f: nc::f_m2 },
    Species { name: "M2",      coefficients: [2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_m2, f: nc::f_m2 },
    Species { name: "lambda2", coefficients: [2.0, 1.0, -2.0, 1.0, 0.0, 0.0, 2.0], u: nc::u_m2, f: nc::f_m2 },
    Species { name: "L2",      coefficients: [2.0, 1.0, 0.0, -1.0, 0.0, 0.0, 2.0], u: nc::u_l2, f: nc::f_l2 },
    Species { name: "T2",      coefficients: [2.0, 2.0, -3.0, 0.0, 0.0, 1.0, 0.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "S2",      coefficients: [2.0, 2.0, -2.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "R2",      coefficients: [2.0, 2.0, -1.0, 0.0, 0.0, -1.0, 2.0], u: nc::u_zero, f: nc::f_unity },
    Species { name: "K2",      coefficients: [2.0, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_k2, f: nc::f_k2 },
    // Third-diurnal
    Species { name: "M3", coefficients: [3.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0], u: nc::u_m3, f: nc::f_m3 },
];

pub(crate) fn find(name: &str) -> Option<&'static Species> {
    SPECIES.iter().find(|s| s.name.eq_ignore_ascii_case(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::astro;

    /// Constituent speeds at T=0 exactly (J2000.0, unix 946728000 =
    /// 2000-01-01T12:00:00Z, chosen so JD = 2451545.0 exactly) against
    /// independently published invariant values, in degrees/hour. These
    /// are physical constants, not station-specific — this test
    /// verifies the astro/species machinery itself, independent of any
    /// real station data (which this repo doesn't have yet).
    #[test]
    fn known_constituent_speeds_at_j2000() {
        let unix_j2000 = 946_728_000_i64;
        let a = astro(unix_j2000);
        let speeds = a.spanning_speeds();

        let speed_of = |name: &str| -> f64 {
            let sp = find(name).unwrap();
            sp.coefficients
                .iter()
                .zip(speeds.iter())
                .map(|(c, s)| c * s)
                .sum()
        };

        let expected: &[(&str, f64)] = &[
            ("M2", 28.9841042),
            ("S2", 30.0000000),
            ("K1", 15.0410686),
            ("O1", 13.9430356),
            ("N2", 28.4397295),
            ("K2", 30.0821373),
            ("Q1", 13.3986609),
            ("P1", 14.9589314),
        ];

        for (name, want) in expected {
            let got = speed_of(name);
            assert!(
                (got - want).abs() < 1e-4,
                "{name} speed: got {got}, want {want}"
            );
        }
    }
}
