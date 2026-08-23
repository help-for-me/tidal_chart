use tidal_engine::{Constituent, Station};

#[test]
fn station_holds_constituents() {
    let constituent = Constituent::new("M2", 1.2, 45.0);
    let station = Station::new(
        "test-station",
        "Test Station",
        49.28,
        -123.12,
        0.0,
        vec![constituent.clone()],
    );

    assert_eq!(station.constituents, vec![constituent]);
}

// TODO: once HarmonicPredictor::water_level and ::extrema are
// implemented, add regression tests against a station with known
// published predictions (e.g. a NOAA reference station) to validate
// the constituent summation, nodal-factor math, and extrema finding.
