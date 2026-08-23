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

    /// High and low tide events (local extrema of `waterLevel(at:)`)
    /// between `startDate` and `endDate`, in chronological order.
    ///
    /// The summed curve is a sum of cosines with different periods, so
    /// extrema have no closed form — this requires sampling
    /// `waterLevel(at:)` and refining around sign changes in its slope
    /// (e.g. bisection or Newton's method on a numerical derivative),
    /// not a formula.
    ///
    /// - Not yet implemented.
    public func extrema(from startDate: Date, to endDate: Date) -> [TideExtremum] {
        fatalError("HarmonicPredictor.extrema(from:to:) not yet implemented")
    }
}
