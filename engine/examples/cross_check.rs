//! Not part of the library's public API — a throwaway harness for
//! cross-checking this engine's output against pytides (an independent
//! Python implementation) numerically. See `docs/VALIDATION.md`.
//!
//! Usage: `cargo run --example cross_check -- water_level <start_unix> <hours>`
//!        `cargo run --example cross_check -- extrema <start_unix> <hours>`
//! Prints CSV to stdout in the same shape as the Python side's output,
//! for an all-23-species station with amplitude 1.0 / phase 0.0 each —
//! deliberately not real station data, just a shared synthetic fixture.

use tidal_engine::{Constituent, HarmonicPredictor, Station};

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mode = args.get(1).expect("mode: water_level|extrema");
    let start: i64 = args.get(2).expect("start_unix").parse().unwrap();
    let hours: i64 = args.get(3).expect("hours").parse().unwrap();

    let predictor = HarmonicPredictor::new(synthetic_station());

    match mode.as_str() {
        "water_level" => {
            let mut t = start;
            let end = start + hours * 3600;
            while t < end {
                println!("{},{:.10}", t, predictor.water_level(t));
                t += 3600;
            }
        }
        "extrema" => {
            let end = start + hours * 3600;
            for e in predictor.extrema(start, end) {
                let kind = match e.kind {
                    tidal_engine::TideExtremumKind::High => "H",
                    tidal_engine::TideExtremumKind::Low => "L",
                };
                println!("{},{:.10},{}", e.unix_time_seconds, e.height, kind);
            }
        }
        other => panic!("unknown mode: {other}"),
    }
}
