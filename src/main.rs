mod pcgbg_dist;
mod pcgbg_noise;

#[cfg(test)]
use approx::assert_relative_eq;
use image::ImageBuffer;
use ndarray::Array2;
use noise::NoiseFn;
use pcgbg_dist::{DistanceEntry, DistanceEntryDistribution, Vec2D};
use pcgbg_noise::{Noise, NoiseDistribution};
use rand::distributions::Distribution;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(
        help = "Path to write generated image",
        long,
        short,
        parse(from_os_str)
    )]
    output_path: PathBuf,
    #[structopt(help = "Image width", long)]
    width: usize,
    #[structopt(help = "Image height", long)]
    height: usize,
    #[structopt(help = "Noise scale", long)]
    scale: f64,
}

fn main() {
    let opts = Opts::from_args();
    let width = opts.width;
    let height = opts.height;
    let scale = opts.scale;

    let seed = get_time_seed();
    let mut rng = SmallRng::seed_from_u64(seed as u64);

    let noise_distribution = NoiseDistribution { scale };
    let noise_r = noise_distribution.sample(&mut rng);
    let noise_g = noise_distribution.sample(&mut rng);
    let noise_b = noise_distribution.sample(&mut rng);

    let dist_entry_distribution = DistanceEntryDistribution { width, height };
    let dist_entry_r = dist_entry_distribution.sample(&mut rng);
    let dist_entry_g = dist_entry_distribution.sample(&mut rng);
    let dist_entry_b = dist_entry_distribution.sample(&mut rng);

    let mut vals_r = Array2::<f64>::zeros((width, height));
    let mut vals_g = Array2::<f64>::zeros((width, height));
    let mut vals_b = Array2::<f64>::zeros((width, height));

    fill_with_distance_entry(&mut vals_r, &dist_entry_r);
    fill_with_distance_entry(&mut vals_g, &dist_entry_g);
    fill_with_distance_entry(&mut vals_b, &dist_entry_b);

    add_noise(&mut vals_r, &noise_r);
    add_noise(&mut vals_g, &noise_g);
    add_noise(&mut vals_b, &noise_b);

    normalize(&mut vals_r);
    normalize(&mut vals_g);
    normalize(&mut vals_b);

    let image = ImageBuffer::from_fn(width as u32, height as u32, |i, j| {
        let x = i as usize;
        let y = j as usize;

        let r = scale_float_to_u8(vals_r[[x, y]]);
        let g = scale_float_to_u8(vals_g[[x, y]]);
        let b = scale_float_to_u8(vals_b[[x, y]]);
        image::Rgb([r, g, b])
    });
    image.save(opts.output_path).unwrap();
}

fn normalized<F: num_traits::Float>(val: F, min: F, max: F) -> F {
    (val - min) / (max - min)
}

#[test]
fn test_normalized() {
    assert_relative_eq!(0.5, normalized(2.0, 1.0, 3.0));
}

fn normalize(vals: &mut Array2<f64>) {
    let mut min = std::f64::MAX;
    let mut max = std::f64::MIN;
    for val in vals.iter() {
        min = min.min(*val);
        max = max.max(*val);
    }
    for val in vals.iter_mut() {
        *val = normalized(*val, min, max);
    }
}

fn fill_with_distance_entry(vals: &mut Array2<f64>, dist_entry: &DistanceEntry) {
    let mut min = std::f64::MAX;
    let mut max = std::f64::MIN;
    for (x, y) in ndarray::indices(vals.raw_dim()) {
        let dist = dist_entry.distance(Vec2D::new(x as f64, y as f64));
        min = min.min(dist);
        max = max.max(dist);
    }
    assert!(min < max);
    for (idx, val) in vals.indexed_iter_mut() {
        let dist = dist_entry.distance(Vec2D::new(idx.0 as f64, idx.1 as f64));
        *val += normalized(dist, min, max);
    }
}

fn add_noise(vals: &mut Array2<f64>, noise: &Noise) {
    for (idx, val) in vals.indexed_iter_mut() {
        *val += noise.get([idx.0 as f64, idx.1 as f64]) * 0.1;
    }
}

fn scale_float_to_u8(val: f64) -> u8 {
    (val * std::u8::MAX as f64) as u8
}

fn get_time_seed() -> u128 {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_nanos()
}
