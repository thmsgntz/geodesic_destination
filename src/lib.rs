//! Spherical direct geodesic (destination point) calculation.
//!
//! This crate solves the *direct* geodesic problem on a spherical Earth model:
//! given a start point (latitude/longitude in **radians**), a distance in meters,
//! and a bearing in radians (North = 0, increasing clockwise), compute the
//! destination point.
//!
//! # Example
//! ```
//! use geodesic_destination::{destination, LatLon, EARTH_RADIUS_M};
//! use std::f64::consts::PI;
//!
//! let start = LatLon::new(0.0, 0.0);
//! let dest = destination(start, EARTH_RADIUS_M * (PI / 2.0), PI / 2.0);
//! assert!((dest.lat - 0.0).abs() < 1e-10);
//! assert!((dest.lon - (PI / 2.0)).abs() < 1e-10);
//! ```

use std::f64::consts::PI;

/// Mean Earth radius in meters.
pub const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Latitude/longitude pair in radians.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLon {
    /// Latitude in radians.
    pub lat: f64,
    /// Longitude in radians.
    pub lon: f64,
}

impl LatLon {
    /// Creates a new `LatLon` from radians.
    #[must_use]
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }
}

/// Returns the destination point using the mean Earth radius.
#[must_use]
pub fn destination(start: LatLon, distance_m: f64, bearing_rad: f64) -> LatLon {
    destination_with_radius(start, distance_m, bearing_rad, EARTH_RADIUS_M)
}

/// Returns the destination point using a custom spherical radius.
#[must_use]
pub fn destination_with_radius(
    start: LatLon,
    distance_m: f64,
    bearing_rad: f64,
    radius_m: f64,
) -> LatLon {
    if distance_m == 0.0 {
        return start;
    }

    // Angular distance in radians.
    let delta = distance_m / radius_m;

    let sin_lat1 = start.lat.sin();
    let cos_lat1 = start.lat.cos();
    let sin_delta = delta.sin();
    let cos_delta = delta.cos();

    // Spherical trigonometry formulae with bearing measured clockwise from North.
    let sin_lat2 = sin_lat1 * cos_delta + cos_lat1 * sin_delta * bearing_rad.cos();
    let lat2 = clamp(sin_lat2, -1.0, 1.0).asin();

    let y = bearing_rad.sin() * sin_delta * cos_lat1;
    let x = cos_delta - sin_lat1 * lat2.sin();
    let lon2 = normalize_lon(start.lon + y.atan2(x));

    LatLon::new(lat2, lon2)
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

fn normalize_lon(lon: f64) -> f64 {
    let mut wrapped = (lon + PI) % (2.0 * PI);
    if wrapped < 0.0 {
        wrapped += 2.0 * PI;
    }
    wrapped - PI
}
