//! Astronomical elements needed for the harmonic tide method: the
//! equilibrium-argument spanning set and the auxiliary lunar-node
//! quantities used by nodal corrections (`crate::nodal`).
//!
//! Formulas are Meeus's "Astronomical Algorithms" polynomial
//! approximations (as used for Schureman's tidal equations in NOAA
//! Special Publication No. 98) — public-domain astronomy, not derived
//! from XTide's or pytides' own code. The numeric coefficients were
//! cross-checked against pytides (MIT-licensed, github.com/sam-cox/pytides)
//! as a working reference, and the resulting constituent speeds verified
//! against independently published invariant values (M2 = 28.9841042,
//! S2 = 30.0 exactly, K1 = 15.0410686, O1 = 13.9430356 degrees/hour) —
//! see the regression test in `crate::species`.

pub(crate) const D2R: f64 = std::f64::consts::PI / 180.0;
pub(crate) const R2D: f64 = 180.0 / std::f64::consts::PI;

/// An astronomical element's current value (degrees, wrapped to
/// `[0, 360)`) and rate of change (degrees/hour).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Element {
    pub value: f64,
    pub speed: f64,
}

/// Everything needed to evaluate a constituent's equilibrium argument
/// and nodal corrections at one instant.
pub(crate) struct Astro {
    pub t_h_s: Element,
    pub s: Element,
    pub h: Element,
    pub p: Element,
    pub n: Element,
    pub pp: Element,
    pub ninety: Element,
    pub omega: Element,
    pub i: Element,
    /// Schureman's I, xi, nu, nu', nu'' and P — auxiliary quantities
    /// derived from N, i, omega (Table 6 in Schureman). Slowly varying;
    /// speeds aren't needed for these.
    pub big_i: f64,
    pub xi: f64,
    pub nu: f64,
    pub nup: f64,
    pub nupp: f64,
    pub p_cap: f64,
}

impl Astro {
    /// Equilibrium-argument spanning set, in the conventional order
    /// `[T+h-s, s, h, p, N, pp, 90]` — a constituent's equilibrium
    /// argument/speed is the dot product of its coefficients with this.
    pub(crate) fn spanning_values(&self) -> [f64; 7] {
        [
            self.t_h_s.value,
            self.s.value,
            self.h.value,
            self.p.value,
            self.n.value,
            self.pp.value,
            self.ninety.value,
        ]
    }

    pub(crate) fn spanning_speeds(&self) -> [f64; 7] {
        [
            self.t_h_s.speed,
            self.s.speed,
            self.h.speed,
            self.p.speed,
            self.n.speed,
            self.pp.speed,
            self.ninety.speed,
        ]
    }
}

fn s2d(degrees: f64, arcmin: f64, arcsec: f64) -> f64 {
    degrees + arcmin / 60.0 + arcsec / 3600.0
}

fn polynomial(coefficients: &[f64], t: f64) -> f64 {
    coefficients
        .iter()
        .enumerate()
        .map(|(i, c)| c * t.powi(i as i32))
        .sum()
}

fn d_polynomial(coefficients: &[f64], t: f64) -> f64 {
    coefficients
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, c)| c * (i as f64) * t.powi(i as i32 - 1))
        .sum()
}

/// Julian Day Number from a Unix timestamp (seconds). JD 2440587.5 is
/// the well-known Unix epoch (1970-01-01T00:00:00Z).
fn julian_day(unix_time_seconds: i64) -> f64 {
    unix_time_seconds as f64 / 86400.0 + 2_440_587.5
}

/// Julian centuries since J2000.0 (Meeus formula 11.1).
fn julian_centuries(unix_time_seconds: i64) -> f64 {
    (julian_day(unix_time_seconds) - 2_451_545.0) / 36525.0
}

fn terrestrial_obliquity_coefficients() -> Vec<f64> {
    // Meeus formula 21.3.
    let raw = [
        s2d(23.0, 26.0, 21.448),
        -s2d(0.0, 0.0, 4680.93),
        -s2d(0.0, 0.0, 1.55),
        s2d(0.0, 0.0, 1999.25),
        -s2d(0.0, 0.0, 51.38),
        -s2d(0.0, 0.0, 249.67),
        -s2d(0.0, 0.0, 39.05),
        s2d(0.0, 0.0, 7.12),
        s2d(0.0, 0.0, 27.87),
        s2d(0.0, 0.0, 5.79),
        s2d(0.0, 0.0, 2.45),
    ];
    // Meeus 21.3 is a polynomial in U = T/100; rescale to a plain
    // polynomial in T.
    raw.iter()
        .enumerate()
        .map(|(i, c)| c * 0.01f64.powi(i as i32))
        .collect()
}

fn solar_perigee_coefficients() -> [f64; 4] {
    // Meeus 24.2 minus 24.3.
    [
        280.46645 - 357.52910,
        36000.76932 - 35999.05030,
        0.0003032 + 0.0001559,
        0.00000048,
    ]
}

fn solar_longitude_coefficients() -> [f64; 3] {
    // Meeus formula 24.2.
    [280.46645, 36000.76983, 0.0003032]
}

fn lunar_inclination_coefficients() -> [f64; 1] {
    // Near-constant; mean inclination of the lunar orbit to the ecliptic.
    [5.145]
}

fn lunar_longitude_coefficients() -> [f64; 4] {
    // Meeus formula 45.1.
    [
        218.3164591,
        481267.88134236,
        -0.0013268,
        1.0 / 538841.0 - 1.0 / 65194000.0,
    ]
}

fn lunar_node_coefficients() -> [f64; 5] {
    // Meeus formula 45.7 — longitude of the moon's ascending node.
    [
        125.0445550,
        -1934.1361849,
        0.0020762,
        1.0 / 467410.0,
        -1.0 / 60616000.0,
    ]
}

fn lunar_perigee_coefficients() -> [f64; 5] {
    // Meeus, formula directly preceding 45.7.
    [
        83.3532430,
        4069.0137111,
        -0.0103238,
        -1.0 / 80053.0,
        1.0 / 18999000.0,
    ]
}

fn wrap360(deg: f64) -> f64 {
    deg.rem_euclid(360.0)
}

// Schureman Table 6: I, xi, nu, nu', nu'' as functions of N, i, omega.

fn big_i(n: f64, i: f64, omega: f64) -> f64 {
    let (n, i, omega) = (D2R * n, D2R * i, D2R * omega);
    let cos_i = i.cos() * omega.cos() - i.sin() * omega.sin() * n.cos();
    R2D * cos_i.acos()
}

fn xi_nu(n: f64, i: f64, omega: f64) -> (f64, f64) {
    let (n, i, omega) = (D2R * n, D2R * i, D2R * omega);
    let e1 = ((0.5 * (omega - i)).cos() / (0.5 * (omega + i)).cos() * (0.5 * n).tan()).atan();
    let e2 = ((0.5 * (omega - i)).sin() / (0.5 * (omega + i)).sin() * (0.5 * n).tan()).atan();
    let e1 = e1 - 0.5 * n;
    let e2 = e2 - 0.5 * n;
    (-(e1 + e2) * R2D, (e1 - e2) * R2D)
}

// Schureman equation 224.
fn nup(n: f64, i: f64, omega: f64) -> f64 {
    let big_i = D2R * big_i(n, i, omega);
    let (_, nu) = xi_nu(n, i, omega);
    let nu = D2R * nu;
    R2D * (((2.0 * big_i).sin() * nu.sin()) / ((2.0 * big_i).sin() * nu.cos() + 0.3347)).atan()
}

// Schureman equation 232.
fn nupp(n: f64, i: f64, omega: f64) -> f64 {
    let big_i = D2R * big_i(n, i, omega);
    let (_, nu) = xi_nu(n, i, omega);
    let nu = D2R * nu;
    let tan_2_nupp = (big_i.sin().powi(2) * (2.0 * nu).sin())
        / (big_i.sin().powi(2) * (2.0 * nu).cos() + 0.0727);
    R2D * 0.5 * tan_2_nupp.atan()
}

pub(crate) fn astro(unix_time_seconds: i64) -> Astro {
    let t = julian_centuries(unix_time_seconds);
    // Polynomials are in T (Julian centuries); speeds are more useful
    // in degrees/hour.
    let d_t_d_hour = 1.0 / (24.0 * 365.25 * 100.0);

    let element = |coefficients: &[f64]| Element {
        value: wrap360(polynomial(coefficients, t)),
        speed: d_polynomial(coefficients, t) * d_t_d_hour,
    };

    let s = element(&lunar_longitude_coefficients());
    let h = element(&solar_longitude_coefficients());
    let p = element(&lunar_perigee_coefficients());
    let n = element(&lunar_node_coefficients());
    let pp = element(&solar_perigee_coefficients());
    let ninety = Element {
        value: 90.0,
        speed: 0.0,
    };
    let omega = element(&terrestrial_obliquity_coefficients());
    let i = element(&lunar_inclination_coefficients());

    let jd = julian_day(unix_time_seconds);
    let hour_value = (jd - jd.floor()) * 360.0;
    let hour_speed = 15.0; // Earth's rotation: 360 degrees / 24 hours.
    let t_h_s = Element {
        value: wrap360(hour_value + h.value - s.value),
        speed: hour_speed + h.speed - s.speed,
    };

    let big_i_v = wrap360(big_i(n.value, i.value, omega.value));
    let (xi_v, nu_v) = xi_nu(n.value, i.value, omega.value);
    let (xi_v, nu_v) = (wrap360(xi_v), wrap360(nu_v));
    let nup_v = wrap360(nup(n.value, i.value, omega.value));
    let nupp_v = wrap360(nupp(n.value, i.value, omega.value));
    let p_cap = wrap360(p.value - xi_v);

    Astro {
        t_h_s,
        s,
        h,
        p,
        n,
        pp,
        ninety,
        omega,
        i,
        big_i: big_i_v,
        xi: xi_v,
        nu: nu_v,
        nup: nup_v,
        nupp: nupp_v,
        p_cap,
    }
}
