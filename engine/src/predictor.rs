use crate::astro::{self, D2R};
use crate::species;
use crate::{Station, TideExtremum, TideExtremumKind};

/// Predicts water level at a station using the harmonic method: a sum
/// of cosine terms, one per constituent, each adjusted by a per-date
/// equilibrium argument and nodal factor.
///
/// Reference: NOAA Special Publication No. 98, "Manual of Harmonic
/// Analysis and Prediction of Tides" (public domain). This is a
/// from-scratch implementation of that published method — not derived
/// from XTide's or any other GPL-licensed source — so the engine stays
/// permissively licensed and safe to embed anywhere.
pub struct HarmonicPredictor {
    pub station: Station,
}

impl HarmonicPredictor {
    pub fn new(station: Station) -> Self {
        Self { station }
    }

    /// Water level and its time-derivative at `unix_time_seconds`,
    /// relative to `station.datum_offset`. Computed together since both
    /// need the same astronomical elements at `unix_time_seconds`.
    ///
    /// Station constituents whose name isn't a recognized species (see
    /// `crate::species`) are silently skipped — not yet validated
    /// against real station data, so this is a documented gap, not a
    /// hidden one.
    fn evaluate(&self, unix_time_seconds: i64) -> (f64, f64) {
        let a = astro::astro(unix_time_seconds);
        let values = a.spanning_values();
        let speeds = a.spanning_speeds();

        let mut height = self.station.datum_offset;
        let mut d_height = 0.0;

        for constituent in &self.station.constituents {
            let Some(sp) = species::find(&constituent.name) else {
                continue;
            };
            let v: f64 = dot(&sp.coefficients, &values);
            let speed_deg_per_hour: f64 = dot(&sp.coefficients, &speeds);
            let u = (sp.u)(&a);
            let f = (sp.f)(&a);
            let phase_rad = D2R * (v + u - constituent.phase_degrees);

            height += constituent.amplitude * f * phase_rad.cos();
            d_height -= constituent.amplitude * f * (D2R * speed_deg_per_hour) * phase_rad.sin();
        }

        (height, d_height)
    }

    /// Predicted water level at `unix_time_seconds`, relative to
    /// `station.datum_offset`.
    pub fn water_level(&self, unix_time_seconds: i64) -> f64 {
        self.evaluate(unix_time_seconds).0
    }

    /// High and low tide events (local extrema of [`Self::water_level`])
    /// between `start_unix_seconds` and `end_unix_seconds`, in
    /// chronological order.
    ///
    /// The summed curve is a sum of cosines with different periods, so
    /// extrema have no closed form: this samples the analytic
    /// derivative at a spacing fine enough to not skip past an
    /// extremum of the fastest constituent present (a quarter of its
    /// period, the same margin used by the reference implementation
    /// this was cross-checked against), then bisects each sign change
    /// down to single-second precision.
    pub fn extrema(&self, start_unix_seconds: i64, end_unix_seconds: i64) -> Vec<TideExtremum> {
        let mut results = Vec::new();
        if end_unix_seconds <= start_unix_seconds {
            return results;
        }

        let probe_speeds = astro::astro(start_unix_seconds).spanning_speeds();
        let max_speed_deg_per_hour = self
            .station
            .constituents
            .iter()
            .filter_map(|c| species::find(&c.name))
            .map(|sp| dot(&sp.coefficients, &probe_speeds).abs())
            .fold(0.0_f64, f64::max);
        if max_speed_deg_per_hour <= 0.0 {
            return results;
        }
        let step_seconds = ((90.0 / max_speed_deg_per_hour) * 3600.0).floor().max(1.0) as i64;

        let mut t_prev = start_unix_seconds - step_seconds;
        let (_, mut d_prev) = self.evaluate(t_prev);
        let mut t = t_prev + step_seconds;

        while t <= end_unix_seconds + step_seconds {
            let (_, d) = self.evaluate(t);
            if d_prev != 0.0 && (d_prev > 0.0) != (d > 0.0) {
                let root = self.bisect_derivative_zero(t_prev, t, d_prev);
                if root >= start_unix_seconds && root <= end_unix_seconds {
                    let (height, _) = self.evaluate(root);
                    let kind = if d_prev > 0.0 {
                        TideExtremumKind::High
                    } else {
                        TideExtremumKind::Low
                    };
                    results.push(TideExtremum {
                        kind,
                        unix_time_seconds: root,
                        height,
                    });
                }
            }
            t_prev = t;
            d_prev = d;
            t += step_seconds;
        }

        results.sort_by_key(|e| e.unix_time_seconds);
        results
    }

    /// Bisects `[lo, hi]` (a bracket where the derivative changes sign,
    /// with derivative `d_lo` at `lo`) down to a 1-second window and
    /// returns the upper bound of that window.
    fn bisect_derivative_zero(&self, mut lo: i64, mut hi: i64, mut d_lo: f64) -> i64 {
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            let (_, d_mid) = self.evaluate(mid);
            if (d_lo > 0.0) == (d_mid > 0.0) {
                lo = mid;
                d_lo = d_mid;
            } else {
                hi = mid;
            }
        }
        hi
    }
}

fn dot(a: &[f64; 7], b: &[f64; 7]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Constituent;

    /// A synthetic single-constituent (M2) station: exercises the full
    /// water_level + extrema pipeline (astro, species lookup, nodal
    /// corrections, bisection) without depending on any real station
    /// data, which this repo doesn't have yet.
    fn m2_only_station() -> Station {
        Station::new(
            "synthetic-m2",
            "Synthetic M2-only station",
            49.28,
            -123.12,
            0.0,
            vec![Constituent::new("M2", 1.0, 0.0)],
        )
    }

    #[test]
    fn extrema_alternate_and_are_spaced_by_roughly_half_the_m2_period() {
        let predictor = HarmonicPredictor::new(m2_only_station());
        let start = 946_728_000_i64; // 2000-01-01T12:00:00Z
        let end = start + 5 * 86_400; // 5 days later

        let extrema = predictor.extrema(start, end);
        assert!(
            extrema.len() >= 8,
            "expected several extrema over 5 days, got {}",
            extrema.len()
        );

        // M2 period is 360/28.9841042 =~ 12.4206 hours; extrema should
        // alternate High/Low, spaced by roughly half that (~6.21 hours).
        for pair in extrema.windows(2) {
            let [a, b] = pair else { unreachable!() };
            assert_ne!(
                std::mem::discriminant(&a.kind),
                std::mem::discriminant(&b.kind),
                "consecutive extrema should alternate High/Low"
            );
            let spacing_hours = (b.unix_time_seconds - a.unix_time_seconds) as f64 / 3600.0;
            assert!(
                (5.5..=7.0).contains(&spacing_hours),
                "expected ~6.21h spacing between M2 extrema, got {spacing_hours}h"
            );
        }

        // With amplitude 1.0 and f close to (but not exactly) 1.0, high
        // tides should be positive and roughly amplitude-sized, lows
        // the mirror image.
        for e in &extrema {
            match e.kind {
                TideExtremumKind::High => {
                    assert!(e.height > 0.8, "high tide too small: {}", e.height)
                }
                TideExtremumKind::Low => {
                    assert!(e.height < -0.8, "low tide too small: {}", e.height)
                }
            }
        }
    }

    #[test]
    fn water_level_matches_extrema_heights() {
        let predictor = HarmonicPredictor::new(m2_only_station());
        let start = 946_728_000_i64;
        let extrema = predictor.extrema(start, start + 86_400);
        for e in &extrema {
            let recomputed = predictor.water_level(e.unix_time_seconds);
            assert!(
                (recomputed - e.height).abs() < 1e-9,
                "water_level({}) = {recomputed}, extrema reported {}",
                e.unix_time_seconds,
                e.height
            );
        }
    }
}
