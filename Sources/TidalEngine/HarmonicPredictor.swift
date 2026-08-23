import Foundation

/// Predicts water level at a station using the harmonic method: a sum
/// of cosine terms, one per constituent, each adjusted by a per-date
/// equilibrium argument and nodal factor.
///
/// Reference: NOAA Special Publication No. 98, "Manual of Harmonic
/// Analysis and Prediction of Tides" (public domain). This is a
/// from-scratch implementation of that published method — not derived
/// from XTide or any other GPL-licensed source — so the engine stays
/// MIT-licensed and safe to embed anywhere, including the App Store.
public struct HarmonicPredictor: Sendable {
    public let station: Station

    public init(station: Station) {
        self.station = station
    }

    /// Predicted water level at `date`, relative to `station.datumOffset`.
    ///
    /// - Not yet implemented. This is where the constituent summation
    ///   (per-date equilibrium argument + nodal factor, per SP 98) goes
    ///   once the buildout starts.
    public func waterLevel(at date: Date) -> Double {
        fatalError("HarmonicPredictor.waterLevel(at:) not yet implemented")
    }
}
