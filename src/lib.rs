#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Spherical direct geodesic (destination point) calculation.
//!
//! This crate solves the *direct* geodesic problem on a spherical Earth model:
//! given a start point, a distance, and a bearing, compute the destination point.
//!
//! ## Units & conventions
//!
//! - Latitude and longitude are in **radians**.
//! - Distance is in **meters**.
//! - Bearing is in **radians**, measured clockwise from geographic North:
//!   `0 = North`, `π/2 = East`.
//!
//! The spherical Earth assumption keeps the API small and deterministic. The
//! constant [`EARTH_RADIUS_M`] provides the mean Earth radius used by
//! [`destination`]; use [`destination_with_radius`] for custom spheres.
//!
//! ## Examples
//!
//! 1000m due North from Paris:
//! ```
//! use geodesic_destination::{destination, LatLon};
//!
//! let start = LatLon::new(48.866667_f64.to_radians(), 2.333333_f64.to_radians());
//! let dest = destination(start, 1_000.0, 0.0);
//!
//! assert!(dest.lat > start.lat);
//! assert!((dest.lon - start.lon).abs() < 1e-6);
//! ```
//!
//! 1000m at 45° (NE) from Paris:
//! ```
//! use geodesic_destination::{destination, LatLon};
//! use std::f64::consts::FRAC_PI_4;
//!
//! let start = LatLon::new(48.866667_f64.to_radians(), 2.333333_f64.to_radians());
//! let dest = destination(start, 1_000.0, FRAC_PI_4);
//!
//! assert!(dest.lat > start.lat);
//! assert!(dest.lon > start.lon);
//! ```

use std::f64::consts::PI;

/// Mean Earth radius in meters for the spherical Earth model.
pub const EARTH_RADIUS_M: f64 = 6_371_000.0;

/// Latitude/longitude pair in radians.
///
/// # Notes
///
/// - Latitude is expected in the range `[-π/2, π/2]`.
/// - Longitude is expected in the range `[-π, π]` and is normalized on output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LatLon {
    /// Latitude in radians, in the range [-π/2, π/2].
    pub lat: f64,
    /// Longitude in radians, in the range [-π, π].
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
///
/// Inputs are in radians (lat/lon), meters (distance), and radians (bearing).
/// Bearing is measured clockwise from geographic North.
///
/// # Notes
///
/// - The output longitude is normalized to `[-π, π]`.
/// - The latitude computation clamps inputs to `asin` to stay within `[-1, 1]`
///   and avoid floating-point drift.
#[must_use]
pub fn destination(start: LatLon, distance_m: f64, bearing_rad: f64) -> LatLon {
    destination_with_radius(start, distance_m, bearing_rad, EARTH_RADIUS_M)
}

/// Returns the destination point using a custom spherical radius.
///
/// Inputs are in radians (lat/lon), meters (distance, radius), and radians
/// (bearing). Bearing is measured clockwise from geographic North.
///
/// # Panics
///
/// Panics if `radius_m` is not positive.
///
/// # Notes
///
/// - The output longitude is normalized to `[-π, π]`.
/// - The latitude computation clamps inputs to `asin` to stay within `[-1, 1]`
///   and avoid floating-point drift.
#[must_use]
pub fn destination_with_radius(
    start: LatLon,
    distance_m: f64,
    bearing_rad: f64,
    radius_m: f64,
) -> LatLon {
    assert!(radius_m > 0.0, "radius_m must be positive");

    if distance_m == 0.0 {
        // Trivial case: no displacement, return the input point unchanged.
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
    // Clamp the argument to asin to avoid NaNs from floating-point drift.
    let lat2 = clamp(sin_lat2, -1.0, 1.0).asin();

    let y = bearing_rad.sin() * sin_delta * cos_lat1;
    let x = cos_delta - sin_lat1 * lat2.sin();
    // Normalize longitude to the conventional [-π, π] interval.
    let lon2 = wrap_pi(start.lon + y.atan2(x));

    LatLon::new(lat2, lon2)
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    value.max(min).min(max)
}

fn wrap_pi(lon: f64) -> f64 {
    let mut wrapped = (lon + PI) % (2.0 * PI);
    if wrapped < 0.0 {
        wrapped += 2.0 * PI;
    }
    wrapped - PI
}
