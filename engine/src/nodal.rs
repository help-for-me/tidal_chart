//! Nodal corrections: the `u` (phase, degrees) and `f` (amplitude,
//! dimensionless) adjustments applied to each constituent to account
//! for the moon's 18.6-year nodal cycle. Formulas are Schureman's (NOAA
//! Special Publication No. 98, public domain). `f_k1`, `u_k1` (via
//! `nup`), `f_k2`, `u_k2` (via `nupp`), `f_l2`, `u_l2`, and `u_m1` have
//! been checked directly against a scanned copy of the actual 1958
//! primary text and confirmed exact (equations 195, 202, 213, 214, 223,
//! 224, 227, 232, 234, 235); `f_m1` was checked the same way and found
//! wrong, then fixed (see its comment). Everything else here is still
//! only cross-checked against pytides' `nodal_corrections.py`
//! (MIT-licensed) as a working reference, not the primary text
//! directly. See `docs/VALIDATION.md` for exactly what's confirmed vs.
//! not, and how the primary text was obtained.

use crate::astro::{Astro, D2R, R2D};

pub(crate) fn f_unity(_a: &Astro) -> f64 {
    1.0
}

pub(crate) fn u_zero(_a: &Astro) -> f64 {
    0.0
}

// Schureman equations 73, 65.
pub(crate) fn f_mm(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let mean = (2.0 / 3.0 - omega.sin().powi(2)) * (1.0 - 1.5 * i.sin().powi(2));
    (2.0 / 3.0 - big_i.sin().powi(2)) / mean
}

// Schureman equations 74, 66.
pub(crate) fn f_mf(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let mean = omega.sin().powi(2) * (0.5 * i).cos().powi(4);
    big_i.sin().powi(2) / mean
}

// Schureman equations 75, 67.
pub(crate) fn f_o1(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let mean = omega.sin() * (0.5 * omega).cos().powi(2) * (0.5 * i).cos().powi(4);
    (big_i.sin() * (0.5 * big_i).cos().powi(2)) / mean
}

// Schureman equations 76, 68.
pub(crate) fn f_j1(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let mean = (2.0 * omega).sin() * (1.0 - 1.5 * i.sin().powi(2));
    (2.0 * big_i).sin() / mean
}

// Schureman equations 77, 69.
pub(crate) fn f_oo1(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let mean = omega.sin() * (0.5 * omega).sin().powi(2) * (0.5 * i).cos().powi(4);
    (big_i.sin() * (0.5 * big_i).sin().powi(2)) / mean
}

// Schureman equations 78, 70.
pub(crate) fn f_m2(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let mean = (0.5 * omega).cos().powi(4) * (0.5 * i).cos().powi(4);
    (0.5 * big_i).cos().powi(4) / mean
}

// Schureman equations 227, 226, 68.
pub(crate) fn f_k1(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let nu = D2R * a.nu;
    let sin2i_cosnu_mean = (2.0 * omega).sin() * (1.0 - 1.5 * i.sin().powi(2));
    let mean = 0.5023 * sin2i_cosnu_mean + 0.1681;
    (0.2523 * (2.0 * big_i).sin().powi(2) + 0.1689 * (2.0 * big_i).sin() * nu.cos() + 0.0283).sqrt()
        / mean
}

// Schureman equations 215, 213, 204.
pub(crate) fn f_l2(a: &Astro) -> f64 {
    let big_i = D2R * a.big_i;
    let p = D2R * a.p_cap;
    let r_a_inv = (1.0 - 12.0 * (0.5 * big_i).tan().powi(2) * (2.0 * p).cos()
        + 36.0 * (0.5 * big_i).tan().powi(4))
    .sqrt();
    f_m2(a) * r_a_inv
}

// Schureman equation 235 (mean, 234; obliquity factor, 71).
//
// 0.2523, confirmed directly against Schureman 1958 p.46 eq.(235) - a
// scanned copy of the primary source (found bundled in
// github.com/JacksonKearl/solunar, OCR is unreliable on 1950s print so
// read visually page-by-page instead - see docs/VALIDATION.md). This
// briefly held 0.2533 mid-development: live-cross-checking against the
// drf5n/pytides Python-3 fork showed better agreement at 0.2533, which
// looked like a fix for a suspected sam-cox/pytides transcription error
// - backwards. The fork has the error; sam-cox/pytides (and the
// original port here) had it right. Checking the primary text is what
// actually settled it, not agreement between two implementations that
// happened to share a bug - see docs/VALIDATION.md for the full story.
pub(crate) fn f_k2(a: &Astro) -> f64 {
    let omega = D2R * a.omega.value;
    let i = D2R * a.i.value;
    let big_i = D2R * a.big_i;
    let nu = D2R * a.nu;
    let sinsq_i_cos2nu_mean = omega.sin().powi(2) * (1.0 - 1.5 * i.sin().powi(2));
    let mean = 0.5023 * sinsq_i_cos2nu_mean + 0.0365;
    (0.2523 * big_i.sin().powi(4) + 0.0367 * big_i.sin().powi(2) * (2.0 * nu).cos() + 0.0013).sqrt()
        / mean
}

// Schureman equation 195 (Q_a term; u via 202, f via 206/207).
//
// The middle term's cos(0.5*I) exponent is -2.0, not the -0.5 both
// pytides variants have (`cos(0.5*I)**(-0.5)` in nodal_corrections.py) -
// confirmed directly against Schureman 1958 p.41 eq.(195), which reads
// "... cos I / cos^2 (1/2 I) cos 2P ...": a squared denominator, not a
// square root. This is a real, shared bug in both pytides variants that
// the live cross-check in pytides_cross_check.rs could not have caught
// (comparing against an implementation with the same bug just agrees
// with the bug) - only checking the primary text found it. This engine
// had inherited it too until this fix.
pub(crate) fn f_m1(a: &Astro) -> f64 {
    let big_i = D2R * a.big_i;
    let p = D2R * a.p_cap;
    let q_a_inv = (0.25
        + 1.5 * big_i.cos() * (2.0 * p).cos() * (0.5 * big_i).cos().powf(-2.0)
        + 2.25 * big_i.cos().powi(2) * (0.5 * big_i).cos().powf(-4.0))
    .sqrt();
    f_o1(a) * q_a_inv
}

// e.g. Schureman equation 149.
pub(crate) fn f_modd(a: &Astro, n: f64) -> f64 {
    f_m2(a).powf(n / 2.0)
}

pub(crate) fn f_m3(a: &Astro) -> f64 {
    f_modd(a, 3.0)
}

// Node factors u, see Schureman Table 2.

pub(crate) fn u_mf(a: &Astro) -> f64 {
    -2.0 * a.xi
}

pub(crate) fn u_o1(a: &Astro) -> f64 {
    2.0 * a.xi - a.nu
}

pub(crate) fn u_j1(a: &Astro) -> f64 {
    -a.nu
}

pub(crate) fn u_oo1(a: &Astro) -> f64 {
    -2.0 * a.xi - a.nu
}

pub(crate) fn u_m2(a: &Astro) -> f64 {
    2.0 * a.xi - 2.0 * a.nu
}

pub(crate) fn u_k1(a: &Astro) -> f64 {
    -a.nup
}

// Schureman 214.
pub(crate) fn u_l2(a: &Astro) -> f64 {
    let big_i = D2R * a.big_i;
    let p = D2R * a.p_cap;
    let cot_half_i_sq = 1.0 / (0.5 * big_i).tan().powi(2);
    let r = R2D * ((2.0 * p).sin() / ((1.0 / 6.0) * cot_half_i_sq - (2.0 * p).cos())).atan();
    2.0 * a.xi - 2.0 * a.nu - r
}

pub(crate) fn u_k2(a: &Astro) -> f64 {
    -2.0 * a.nupp
}

// Schureman 202.
pub(crate) fn u_m1(a: &Astro) -> f64 {
    let big_i = D2R * a.big_i;
    let p = D2R * a.p_cap;
    let q = R2D * (((5.0 * big_i.cos() - 1.0) / (7.0 * big_i.cos() + 1.0)) * p.tan()).atan();
    a.xi - a.nu + q
}

pub(crate) fn u_modd(a: &Astro, n: f64) -> f64 {
    n / 2.0 * u_m2(a)
}

pub(crate) fn u_m3(a: &Astro) -> f64 {
    u_modd(a, 3.0)
}
