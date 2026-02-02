use geodesic_destination::{destination, LatLon, EARTH_RADIUS_M};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

const EPSILON_ZERO: f64 = 1e-12;
const DIST_TOLERANCE_M: f64 = 5.0;
const LON_TOLERANCE_RAD: f64 = 1e-6;
const BEARING_TOLERANCE_RAD: f64 = 1e-3;

fn assert_approx_eq(a: f64, b: f64, eps: f64) {
    assert!((a - b).abs() < eps, "expected {} ≈ {} within {}", a, b, eps);
}

fn haversine_m(a: LatLon, b: LatLon, radius_m: f64) -> f64 {
    let dlat = b.lat - a.lat;
    let dlon = b.lon - a.lon;
    let sin_dlat = (dlat / 2.0).sin();
    let sin_dlon = (dlon / 2.0).sin();

    let h = sin_dlat * sin_dlat + a.lat.cos() * b.lat.cos() * sin_dlon * sin_dlon;
    let c = 2.0 * h.sqrt().atan2((1.0 - h).sqrt());
    radius_m * c
}

fn initial_bearing(a: LatLon, b: LatLon) -> f64 {
    let d_lon = b.lon - a.lon;
    let y = d_lon.sin() * b.lat.cos();
    let x = a.lat.cos() * b.lat.sin() - a.lat.sin() * b.lat.cos() * d_lon.cos();
    let mut bearing = y.atan2(x);
    if bearing < 0.0 {
        bearing += 2.0 * PI;
    }
    bearing
}

fn ang_diff(a: f64, b: f64) -> f64 {
    let mut diff = (a - b + PI) % (2.0 * PI);
    if diff < 0.0 {
        diff += 2.0 * PI;
    }
    (diff - PI).abs()
}

fn paris_start() -> LatLon {
    let lat = 48.866667_f64.to_radians();
    let lon = 2.333333_f64.to_radians();
    LatLon::new(lat, lon)
}

#[test]
fn paris_1000m_due_north() {
    let start = paris_start();
    let dest = destination(start, 1000.0, 0.0);

    assert!(dest.lat > start.lat);
    assert_approx_eq(dest.lon, start.lon, LON_TOLERANCE_RAD);

    let distance = haversine_m(start, dest, EARTH_RADIUS_M);
    assert_approx_eq(distance, 1000.0, DIST_TOLERANCE_M);

    let bearing = initial_bearing(start, dest);
    assert!(ang_diff(bearing, 0.0) < BEARING_TOLERANCE_RAD);
}

#[test]
fn paris_1000m_northeast() {
    let start = paris_start();
    let dest = destination(start, 1000.0, FRAC_PI_4);

    assert!(dest.lat > start.lat);
    assert!(dest.lon > start.lon);

    let distance = haversine_m(start, dest, EARTH_RADIUS_M);
    assert_approx_eq(distance, 1000.0, DIST_TOLERANCE_M);

    let bearing = initial_bearing(start, dest);
    assert!(ang_diff(bearing, FRAC_PI_4) < BEARING_TOLERANCE_RAD);
}

#[test]
fn paris_1000m_due_east() {
    let start = paris_start();
    let dest = destination(start, 1000.0, FRAC_PI_2);

    assert!(dest.lon > start.lon);

    let distance = haversine_m(start, dest, EARTH_RADIUS_M);
    assert_approx_eq(distance, 1000.0, DIST_TOLERANCE_M);

    let bearing = initial_bearing(start, dest);
    assert!(ang_diff(bearing, FRAC_PI_2) < BEARING_TOLERANCE_RAD);
}

#[test]
fn paris_zero_distance_due_north() {
    let start = paris_start();
    let dest = destination(start, 0.0, 0.0);

    assert_approx_eq(dest.lat, start.lat, EPSILON_ZERO);
    assert_approx_eq(dest.lon, start.lon, EPSILON_ZERO);
}

#[test]
fn paris_zero_distance_east() {
    let start = paris_start();
    let dest = destination(start, 0.0, FRAC_PI_2);

    assert_approx_eq(dest.lat, start.lat, EPSILON_ZERO);
    assert_approx_eq(dest.lon, start.lon, EPSILON_ZERO);
}
