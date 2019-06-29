use crate::pcgbg_buf::ValuePlane;
#[cfg(test)]
use approx::assert_relative_eq;
use noisy_float::prelude::*;
use rand::distributions::Distribution;
use rand::Rng;

pub type Vec2D = euclid::Vector2D<f64>;

#[derive(Copy, Clone, rand_derive::Rand)]
pub enum DistanceType {
    Manhattan,
    Euclidean,
    Euclidean2,
    Chebyshev,
    MinXY,
}

fn distance(distance_type: DistanceType, v: Vec2D) -> f64 {
    let abs = v.abs();
    match distance_type {
        DistanceType::Manhattan => abs.x + abs.y,
        DistanceType::Euclidean => abs.x.powi(2) + abs.y.powi(2),
        DistanceType::Euclidean2 => (abs.x.powi(2) + abs.y.powi(2)).sqrt(),
        DistanceType::Chebyshev => std::cmp::max(r64(abs.x), r64(abs.y)).raw(),
        DistanceType::MinXY => std::cmp::min(r64(abs.x), r64(abs.y)).raw(),
    }
}

pub struct DistanceEntry {
    distance_type: DistanceType,
    size: Vec2D,
    max_distance: f64,
    center: Vec2D,
    wrap: bool,
    reverse_distance: bool,
}

pub struct DistanceEntryDistribution {
    pub width: usize,
    pub height: usize,
}

impl Distribution<DistanceEntry> for DistanceEntryDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DistanceEntry {
        DistanceEntry::new(
            rng.gen(),
            Vec2D::new(self.width as f64, self.height as f64),
            Vec2D::new(
                rng.gen_range(0.0, self.width as f64),
                rng.gen_range(0.0, self.height as f64),
            ),
            rng.gen(),
            rng.gen(),
        )
    }
}

impl DistanceEntry {
    pub fn new(
        distance_type: DistanceType,
        size: Vec2D,
        center: Vec2D,
        wrap: bool,
        reverse_distance: bool,
    ) -> Self {
        let max_distance = distance(
            distance_type,
            if wrap {
                size / 2.0
            } else {
                center.max(size - center)
            },
        );
        Self {
            distance_type,
            size,
            max_distance,
            center,
            wrap,
            reverse_distance,
        }
    }

    pub fn distance(&self, v: Vec2D) -> f64 {
        let mut delta = (v - self.center).abs();
        if self.wrap {
            delta = delta.min((v - self.size - self.center).abs());
            delta = delta.min((v + self.size - self.center).abs());
        }
        let dist = distance(self.distance_type, delta);
        if self.reverse_distance {
            self.max_distance - dist
        } else {
            dist
        }
    }
}

impl ValuePlane for DistanceEntry {
    fn val(&self, x: f64, y: f64) -> f64 {
        self.distance(Vec2D::new(x, y))
    }
}

#[test]
fn test_dist_entry() {
    assert_relative_eq!(
        DistanceEntry::new(
            DistanceType::Manhattan,
            Vec2D::new(7.0, 8.0),
            Vec2D::new(1.0, 0.0),
            false,
            false
        )
        .distance(Vec2D::new(3.0, 3.0)),
        5.0
    );
    assert_relative_eq!(
        DistanceEntry::new(
            DistanceType::Manhattan,
            Vec2D::new(7.0, 8.0),
            Vec2D::new(1.0, 0.0),
            true,
            false
        )
        .distance(Vec2D::new(6.0, 7.0)),
        3.0
    );
    assert_relative_eq!(
        DistanceEntry::new(
            DistanceType::Manhattan,
            Vec2D::new(4.0, 2.0),
            Vec2D::new(3.0, 0.0),
            true,
            true,
        )
        .distance(Vec2D::new(0.0, 1.0)),
        1.0
    );
}
