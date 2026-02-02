# geodesic_destination

`geodesic_destination` solves the direct geodesic problem on a spherical Earth.
Given a start point (latitude/longitude in radians), a distance in meters, and a
bearing in radians measured clockwise from geographic North, it computes the
resulting destination point (latitude/longitude in radians).

## Features

- Simple, deterministic API
- Spherical Earth model
- No external dependencies
- Suitable for navigation, simulation, robotics, and GIS-lite tasks

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
geodesic_destination = "0.1"
```

## Usage example

```rust
use geodesic_destination::{destination, LatLon, EARTH_RADIUS_M};
use std::f64::consts::FRAC_PI_4;

let start = LatLon::new(48.866667_f64.to_radians(), 2.333333_f64.to_radians());
let distance_m = 1_000.0;
let bearing = FRAC_PI_4; // 45° clockwise from North, in radians.

let dest = destination(start, distance_m, bearing);

// dest.lat and dest.lon are in radians.
println!("lat: {}, lon: {}", dest.lat, dest.lon);
```

## Mathematical model

This crate uses a spherical Earth approximation and solves the direct geodesic
problem: given a start point, distance, and bearing, compute the destination.
The constant `EARTH_RADIUS_M` provides the mean Earth radius in meters used by
the default `destination` function. For custom spheres, use the radius-aware
function provided by the crate.

## Angle & coordinate conventions

- Latitude ∈ [-π/2, π/2]
- Longitude ∈ [-π, π]
- Bearing: 0 = North, π/2 = East, increasing clockwise

All angles are in radians and distances are in meters.

## Accuracy & limitations

- Uses a spherical Earth model (not WGS84/ellipsoidal)
- Appropriate for short to medium distances
- Not suitable for high-precision surveying or geodetic applications
- `no_std` is not enabled yet because the crate currently relies on `std` trigonometry

## Testing

The test suite includes:

- Real-world reference-point checks using Paris
- Distance validation via haversine calculations
- Bearing validation for cardinal and diagonal moves

Run tests with:

```bash
cargo test
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
