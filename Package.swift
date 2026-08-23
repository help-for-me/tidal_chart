// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "TidalEngine",
    platforms: [
        .iOS(.v17),
        .macOS(.v14),
    ],
    products: [
        .library(name: "TidalEngine", targets: ["TidalEngine"])
    ],
    targets: [
        .target(name: "TidalEngine"),
        .testTarget(name: "TidalEngineTests", dependencies: ["TidalEngine"]),
    ]
)
