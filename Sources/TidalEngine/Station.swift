import Foundation

/// A tide station: location plus the harmonic constituents needed to
/// predict water level there.
public struct Station: Equatable, Sendable {
    /// Stable identifier from the source dataset (e.g. NOAA/CHS station ID).
    public let id: String

    public let name: String
    public let latitude: Double
    public let longitude: Double

    /// Height (in the same units as each constituent's amplitude) that
    /// predictions are measured relative to, e.g. Mean Lower Low Water.
    public let datumOffset: Double

    public let constituents: [Constituent]

    public init(
        id: String,
        name: String,
        latitude: Double,
        longitude: Double,
        datumOffset: Double,
        constituents: [Constituent]
    ) {
        self.id = id
        self.name = name
        self.latitude = latitude
        self.longitude = longitude
        self.datumOffset = datumOffset
        self.constituents = constituents
    }
}
