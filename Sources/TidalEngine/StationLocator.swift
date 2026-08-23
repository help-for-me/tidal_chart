import Foundation

/// Finds the nearest station to a GPS coordinate, from whatever station
/// data is bundled/loaded.
public struct StationLocator: Sendable {
    public let stations: [Station]

    public init(stations: [Station]) {
        self.stations = stations
    }

    /// Nearest station to (`latitude`, `longitude`) by great-circle
    /// distance, or `nil` if `stations` is empty.
    ///
    /// - Not yet implemented.
    public func nearest(toLatitude latitude: Double, longitude: Double) -> Station? {
        fatalError("StationLocator.nearest(toLatitude:longitude:) not yet implemented")
    }
}
