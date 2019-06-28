use noisy_float::prelude::*;

type Vec2D = euclid::Vector2D<R64>;

#[derive(Copy, Clone)]
pub enum DistanceType {
    Manhattan,
    Euclidean,
    Euclidean2,
    Chebyshev,
    MinXY,
}

pub fn distance(distance_type: DistanceType, v: Vec2D) -> R64 {
    let abs = v.abs();
    match distance_type {
        DistanceType::Manhattan => abs.x + abs.y,
        DistanceType::Euclidean => abs.x.powi(2) + abs.y.powi(2),
        DistanceType::Euclidean2 => (abs.x.powi(2) + abs.y.powi(2)).sqrt(),
        DistanceType::Chebyshev => std::cmp::max(abs.x, abs.y),
        DistanceType::MinXY => std::cmp::min(abs.x, abs.y),
    }
}

pub struct DistanceEntry {
    distance_type: DistanceType,
    size: Vec2D,
    max_distance: R64,
    center: Vec2D,
    wrap: bool,
    reverse_distance: bool,
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
                size / r64(2.0)
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

    fn distance(&self, v: Vec2D) -> R64 {
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

    pub fn scaled_distance(&self, v: Vec2D) -> R64 {
        self.distance(v) / self.max_distance
    }
}

fn approx_eq(a: R64, b: R64) -> bool {
    (a - b).abs() < r64(0.00000001)
}

#[test]
fn test_dist_entry() {
    assert!(approx_eq(
        DistanceEntry::new(
            DistanceType::Manhattan,
            Vec2D::new(r64(7.0), r64(8.0)),
            Vec2D::new(r64(1.0), r64(0.0)),
            false,
            false
        )
        .distance(Vec2D::new(r64(3.0), r64(3.0))),
        r64(5.0)
    ));
    assert!(approx_eq(
        DistanceEntry::new(
            DistanceType::Manhattan,
            Vec2D::new(r64(7.0), r64(8.0)),
            Vec2D::new(r64(1.0), r64(0.0)),
            true,
            false
        )
        .distance(Vec2D::new(r64(6.0), r64(7.0))),
        r64(3.0)
    ));
    assert!(approx_eq(
        DistanceEntry::new(
            DistanceType::Manhattan,
            Vec2D::new(r64(4.0), r64(2.0)),
            Vec2D::new(r64(3.0), r64(0.0)),
            true,
            true,
        )
        .distance(Vec2D::new(r64(0.0), r64(1.0))),
        r64(1.0)
    ));
}
