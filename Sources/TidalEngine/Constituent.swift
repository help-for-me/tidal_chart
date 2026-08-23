import Foundation

/// A single tidal harmonic constituent (e.g. M2, S2, K1, O1) for one
/// station, as published by a tide authority (NOAA, CHS, etc.).
///
/// These values are empirical, not computed — they come from the
/// station's harmonic constituent data, not derived at runtime.
public struct Constituent: Equatable, Sendable {
    /// Standard constituent name, e.g. "M2", "S2", "K1", "O1".
    public let name: String

    /// Amplitude, in the station's reference units (typically meters).
    public let amplitude: Double

    /// Phase lag, in degrees, relative to the constituent's equilibrium
    /// argument at the station's reference meridian.
    public let phaseDegrees: Double

    public init(name: String, amplitude: Double, phaseDegrees: Double) {
        self.name = name
        self.amplitude = amplitude
        self.phaseDegrees = phaseDegrees
    }
}
