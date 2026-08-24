//! Not part of the library's public API — reads harmonic constants
//! fitted (via a linear least-squares harmonic analysis in Python,
//! using pytides' verified astro()/nodal functions) from real CHS
//! observed-water-level data, and this engine's predictions against
//! both that Python fit's reconstruction and the real observations
//! themselves. See docs/VALIDATION.md (private companion repo) for
//! the full writeup.
//!
//! Usage: `cargo run --example real_station_check -- <constants.txt> <heldout.csv>`
//! constants.txt: first line Z0, then "NAME AMPLITUDE PHASE_DEGREES" per line.
//! heldout.csv: "unix_time_seconds,observed,python_predicted" per line.
//! Prints "unix_time,observed,python_predicted,rust_predicted" CSV to stdout.

use std::env;
use std::fs;

use tidal_engine::{Constituent, HarmonicPredictor, Station};

fn main() {
    let args: Vec<String> = env::args().collect();
    let constants_path = &args[1];
    let heldout_path = &args[2];

    let constants_text = fs::read_to_string(constants_path).unwrap();
    let mut lines = constants_text.lines();
    let z0: f64 = lines.next().unwrap().trim().parse().unwrap();

    let mut constituents = Vec::new();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        let amplitude: f64 = parts.next().unwrap().parse().unwrap();
        let phase: f64 = parts.next().unwrap().parse().unwrap();
        constituents.push(Constituent::new(name, amplitude, phase));
    }

    let station = Station::new("real", "real", 0.0, 0.0, z0, constituents);
    let predictor = HarmonicPredictor::new(station);

    let heldout_text = fs::read_to_string(heldout_path).unwrap();
    for line in heldout_text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split(',');
        let t: i64 = parts.next().unwrap().parse().unwrap();
        let observed: f64 = parts.next().unwrap().parse().unwrap();
        let python_predicted: f64 = parts.next().unwrap().parse().unwrap();
        let rust_predicted = predictor.water_level(t);
        println!("{t},{observed},{python_predicted},{rust_predicted}");
    }
}
