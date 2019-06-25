
use noisy_float::prelude::*;

pub enum DistanceType {
    Manhattan,
    Euclidean,
    Euclidean2,
    Chebyshev,
    MinXY,
}

pub fn distance(dx: R64, dy: R64, dist_type: DistanceType) -> R64 {
    match dist_type {
        DistanceType::Manhattan => dx.abs() + dy.abs(),
        DistanceType::Euclidean => dx.abs().powi(2) + dy.abs().powi(2),
        DistanceType::Euclidean2 => (dx.abs().powi(2) + dy.abs().powi(2)).sqrt(),
        DistanceType::Chebyshev => std::cmp::max(dx.abs() , dy.abs()),
        DistanceType::MinXY => std::cmp::min(dx.abs() , dy.abs()),
    }
}
