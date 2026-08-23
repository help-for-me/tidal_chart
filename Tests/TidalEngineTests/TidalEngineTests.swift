import XCTest

@testable import TidalEngine

final class TidalEngineTests: XCTestCase {
    func testStationHoldsConstituents() {
        let constituent = Constituent(name: "M2", amplitude: 1.2, phaseDegrees: 45)
        let station = Station(
            id: "test-station",
            name: "Test Station",
            latitude: 49.28,
            longitude: -123.12,
            datumOffset: 0,
            constituents: [constituent]
        )

        XCTAssertEqual(station.constituents, [constituent])
    }

    // TODO: once HarmonicPredictor.waterLevel(at:) is implemented, add
    // regression tests against a station with known published
    // predictions (e.g. a NOAA reference station) to validate the
    // constituent summation and nodal-factor math.
}
