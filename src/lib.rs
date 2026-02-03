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
//! use geodesic_destination::test_utils::{angle_diff_deg, azimuth_deg, distance_m};
//!
//! let start = LatLon::new(48.866667_f64.to_radians(), 2.333333_f64.to_radians());
//! let dest = destination(start, 1_000.0, 0.0);
//!
//! let tol_m = 1e-3;
//! let tol_deg = 1e-6;
//! assert!((distance_m(start, dest) - 1_000.0).abs() < tol_m);
//! assert!(angle_diff_deg(azimuth_deg(start, dest), 0.0) < tol_deg);
//! ```
//!
//! 1000m at 45° (NE) from Paris:
//! ```
//! use geodesic_destination::{destination, LatLon};
//! use std::f64::consts::FRAC_PI_4;
//! use geodesic_destination::test_utils::{angle_diff_deg, azimuth_deg, distance_m};
//!
//! let start = LatLon::new(48.866667_f64.to_radians(), 2.333333_f64.to_radians());
//! let dest = destination(start, 1_000.0, FRAC_PI_4);
//!
//! let tol_m = 1e-3;
//! let tol_deg = 1e-6;
//! assert!((distance_m(start, dest) - 1_000.0).abs() < tol_m);
//! assert!(angle_diff_deg(azimuth_deg(start, dest), 45.0) < tol_deg);
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

/// Helpers for validating spherical calculations in tests and doctests.
///
/// This module is intentionally `doc(hidden)` outside of tests to keep the
/// public surface area focused on the main API while still supporting doctests.
#[cfg_attr(not(test), doc(hidden))]
pub mod test_utils {
    use super::{LatLon, EARTH_RADIUS_M};

    /// Returns the great-circle distance between two points in meters.
    ///
    /// This uses the haversine formula to match the crate's spherical Earth model.
    #[must_use]
    pub fn distance_m(p: LatLon, q: LatLon) -> f64 {
        let dlat = q.lat - p.lat;
        let dlon = q.lon - p.lon;
        let sin_dlat = (dlat * 0.5).sin();
        let sin_dlon = (dlon * 0.5).sin();
        let a = sin_dlat * sin_dlat + p.lat.cos() * q.lat.cos() * sin_dlon * sin_dlon;
        // Clamp to avoid tiny floating-point drift outside [0, 1].
        let a = super::clamp(a, 0.0, 1.0);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        EARTH_RADIUS_M * c
    }

    /// Returns the initial azimuth (bearing) from `p` to `s`, in degrees.
    ///
    /// The result is normalized to `[0, 360)`. If the points are identical, this
    /// returns `0.0` to keep comparisons stable in tests.
    #[must_use]
    pub fn azimuth_deg(p: LatLon, s: LatLon) -> f64 {
        let dlon = s.lon - p.lon;
        let y = dlon.sin() * s.lat.cos();
        let x = p.lat.cos() * s.lat.sin() - p.lat.sin() * s.lat.cos() * dlon.cos();

        if x == 0.0 && y == 0.0 {
            return 0.0;
        }

        let bearing_rad = y.atan2(x);
        let bearing_deg = bearing_rad.to_degrees();
        wrap_360(bearing_deg)
    }

    /// Returns the absolute minimal angular difference between two bearings.
    #[must_use]
    pub fn angle_diff_deg(a: f64, b: f64) -> f64 {
        let mut diff = (a - b) % 360.0;
        if diff < -180.0 {
            diff += 360.0;
        } else if diff > 180.0 {
            diff -= 360.0;
        }
        diff.abs()
    }

    fn wrap_360(deg: f64) -> f64 {
        let mut wrapped = deg % 360.0;
        if wrapped < 0.0 {
            wrapped += 360.0;
        }
        wrapped
    }

    #[cfg(test)]
    mod tests {
        use super::{angle_diff_deg, azimuth_deg, distance_m};
        use crate::{LatLon, EARTH_RADIUS_M};

        #[test]
        fn distance_is_symmetric_and_zero() {
            let p = LatLon::new(0.1, -1.2);
            let q = LatLon::new(-0.3, 2.4);
            let dist_pq = distance_m(p, q);
            let dist_qp = distance_m(q, p);

            assert!((dist_pq - dist_qp).abs() < 1e-9);
            assert!(distance_m(p, p).abs() < 1e-9);
        }

        #[test]
        fn distance_matches_simple_case() {
            let p = LatLon::new(0.0, 0.0);
            let q = LatLon::new(0.0, (1.0_f64).to_radians());
            let expected = EARTH_RADIUS_M * (1.0_f64).to_radians();

            assert!((distance_m(p, q) - expected).abs() < 1e-6);
        }

        #[test]
        fn azimuth_handles_cardinal_and_same_point() {
            let p = LatLon::new(0.0, 0.0);
            let east = LatLon::new(0.0, (1.0_f64).to_radians());
            let north = LatLon::new((1.0_f64).to_radians(), 0.0);

            assert!(angle_diff_deg(azimuth_deg(p, east), 90.0) < 1e-6);
            assert!(angle_diff_deg(azimuth_deg(p, north), 0.0) < 1e-6);
            assert_eq!(azimuth_deg(p, p), 0.0);
        }
    }
}
