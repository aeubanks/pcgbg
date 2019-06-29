use noise::{Fbm, ScalePoint, Seedable};
use rand::distributions::Distribution;
use rand::Rng;

pub type Noise = ScalePoint<Fbm>;

pub struct NoiseDistribution {
    pub scale: f64,
}

impl Distribution<Noise> for NoiseDistribution {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Noise {
        let fbm = Fbm::new().set_seed(rng.gen());
        ScalePoint::new(fbm)
            .set_x_scale(self.scale)
            .set_y_scale(self.scale)
    }
}
