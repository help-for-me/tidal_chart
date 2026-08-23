import Foundation

/// A single high or low tide event: a local extremum of the predicted
/// water level curve.
public struct TideExtremum: Equatable, Sendable {
    public enum Kind: Sendable {
        case high
        case low
    }

    public let kind: Kind
    public let date: Date

    /// Predicted water level at `date`, relative to the station's
    /// `datumOffset` — same units as `Constituent.amplitude`.
    public let height: Double

    public init(kind: Kind, date: Date, height: Double) {
        self.kind = kind
        self.date = date
        self.height = height
    }
}
