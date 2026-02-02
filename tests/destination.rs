use geodesic_destination::{destination_with_radius, LatLon, EARTH_RADIUS_M};
use std::f64::consts::PI;

const EPS: f64 = 1e-10;

fn approx_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

#[test]
fn zero_distance_returns_start() {
    let start = LatLon::new(0.3, -1.2);
    let dest = destination_with_radius(start, 0.0, 1.0, EARTH_RADIUS_M);
    assert!(approx_eq(dest.lat, start.lat));
    assert!(approx_eq(dest.lon, start.lon));
}

#[test]
fn east_quarter_turn_on_equator() {
    let start = LatLon::new(0.0, 0.0);
    let distance = EARTH_RADIUS_M * (PI / 2.0);
    let dest = destination_with_radius(start, distance, PI / 2.0, EARTH_RADIUS_M);
    assert!(approx_eq(dest.lat, 0.0));
    assert!(approx_eq(dest.lon, PI / 2.0));
}

#[test]
fn north_quarter_turn_from_equator() {
    let start = LatLon::new(0.0, 0.0);
    let distance = EARTH_RADIUS_M * (PI / 2.0);
    let dest = destination_with_radius(start, distance, 0.0, EARTH_RADIUS_M);
    assert!(approx_eq(dest.lat, PI / 2.0));
    assert!(approx_eq(dest.lon, 0.0));
}
