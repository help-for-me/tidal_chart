//! Parses a `Station` from a simple, dependency-free text format —
//! `key=value` header lines, then one `NAME AMPLITUDE PHASE_DEGREES`
//! line per constituent. No JSON/serde dependency, matching the rest of
//! this crate; deliberately readable/diffable as plain text too.
//!
//! ```text
//! id=7795
//! name=Point Atkinson
//! latitude=49.337
//! longitude=-123.253
//! datum_offset=2.97363474623764
//! M2 0.9194506933296124 32.02906512929405
//! S2 0.21505422511119143 71.15042979444038
//! ```
//!
//! `#`-prefixed and blank lines are ignored anywhere. This is the
//! engine-side half of the data pipeline; the actual bundled station
//! files (real, real-station-derived data) aren't part of this public
//! repo — see `docs/DATA.md` and `docs/VALIDATION.md` for why.

use std::fmt;

use crate::{Constituent, Station};

#[derive(Debug, Clone, PartialEq, uniffi::Error)]
#[uniffi(flat_error)]
pub enum ParseStationError {
    MissingField(&'static str),
    InvalidNumber { field: &'static str, line: usize },
    InvalidConstituentLine { line: usize },
}

impl fmt::Display for ParseStationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseStationError::MissingField(field) => write!(f, "missing required field: {field}"),
            ParseStationError::InvalidNumber { field, line } => {
                write!(f, "invalid number for {field} on line {line}")
            }
            ParseStationError::InvalidConstituentLine { line } => {
                write!(
                    f,
                    "invalid constituent line {line} (expected \"NAME AMPLITUDE PHASE\")"
                )
            }
        }
    }
}

impl std::error::Error for ParseStationError {}

/// Parses a `Station` from the text format documented on this module.
#[uniffi::export]
pub fn parse_station(text: &str) -> Result<Station, ParseStationError> {
    let mut id: Option<String> = None;
    let mut name: Option<String> = None;
    let mut latitude: Option<f64> = None;
    let mut longitude: Option<f64> = None;
    let mut datum_offset: Option<f64> = None;
    let mut constituents = Vec::new();

    for (i, raw_line) in text.lines().enumerate() {
        let line_number = i + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "id" => id = Some(value.to_string()),
                "name" => name = Some(value.to_string()),
                "latitude" => {
                    latitude =
                        Some(
                            value
                                .parse()
                                .map_err(|_| ParseStationError::InvalidNumber {
                                    field: "latitude",
                                    line: line_number,
                                })?,
                        )
                }
                "longitude" => {
                    longitude =
                        Some(
                            value
                                .parse()
                                .map_err(|_| ParseStationError::InvalidNumber {
                                    field: "longitude",
                                    line: line_number,
                                })?,
                        )
                }
                "datum_offset" => {
                    datum_offset =
                        Some(
                            value
                                .parse()
                                .map_err(|_| ParseStationError::InvalidNumber {
                                    field: "datum_offset",
                                    line: line_number,
                                })?,
                        )
                }
                _ => {}
            }
            continue;
        }

        let mut parts = line.split_whitespace();
        let (Some(name), Some(amplitude), Some(phase)) = (parts.next(), parts.next(), parts.next())
        else {
            return Err(ParseStationError::InvalidConstituentLine { line: line_number });
        };
        let amplitude: f64 = amplitude
            .parse()
            .map_err(|_| ParseStationError::InvalidConstituentLine { line: line_number })?;
        let phase: f64 = phase
            .parse()
            .map_err(|_| ParseStationError::InvalidConstituentLine { line: line_number })?;
        constituents.push(Constituent::new(name, amplitude, phase));
    }

    Ok(Station::new(
        id.ok_or(ParseStationError::MissingField("id"))?,
        name.ok_or(ParseStationError::MissingField("name"))?,
        latitude.ok_or(ParseStationError::MissingField("latitude"))?,
        longitude.ok_or(ParseStationError::MissingField("longitude"))?,
        datum_offset.ok_or(ParseStationError::MissingField("datum_offset"))?,
        constituents,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_well_formed_station() {
        let text = "\
            id=7795\n\
            name=Point Atkinson\n\
            latitude=49.337\n\
            longitude=-123.253\n\
            datum_offset=2.97\n\
            # a comment, and a blank line follow\n\
            \n\
            M2 0.92 32.03\n\
            S2 0.22 71.15\n\
        ";
        let station = parse_station(text).unwrap();
        assert_eq!(station.id, "7795");
        assert_eq!(station.name, "Point Atkinson");
        assert_eq!(station.latitude, 49.337);
        assert_eq!(station.longitude, -123.253);
        assert_eq!(station.datum_offset, 2.97);
        assert_eq!(station.constituents.len(), 2);
        assert_eq!(station.constituents[0].name, "M2");
        assert_eq!(station.constituents[0].amplitude, 0.92);
        assert_eq!(station.constituents[0].phase_degrees, 32.03);
    }

    #[test]
    fn missing_field_is_an_error_not_a_panic() {
        let text = "id=7795\nname=X\nlatitude=1.0\nlongitude=2.0\n";
        assert_eq!(
            parse_station(text),
            Err(ParseStationError::MissingField("datum_offset"))
        );
    }

    #[test]
    fn malformed_constituent_line_is_an_error() {
        let text = "\
            id=1\nname=X\nlatitude=1.0\nlongitude=2.0\ndatum_offset=0.0\n\
            M2 not-a-number 32.0\n\
        ";
        assert!(matches!(
            parse_station(text),
            Err(ParseStationError::InvalidConstituentLine { .. })
        ));
    }
}
