mod dist;

use dist::{DistanceEntry, Vec2D};
use noise::{Fbm, MultiFractal, NoiseFn, ScalePoint, Seedable};
use std::path::PathBuf;
use structopt::StructOpt;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

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
    width: u32,
    #[structopt(help = "Image height", long)]
    height: u32,
    #[structopt(help = "Noise scale", long)]
    scale: f64,
}

fn main() {
    let opts = Opts::from_args();
    let width = opts.width;
    let height = opts.height;
    let scale = opts.scale;
    let seed = get_time_seed();
    let mut rand = SmallRng::seed_from_u64(seed as u64);
    let noise_r = create_noise(seed ^ 0, scale);
    let noise_g = create_noise(seed ^ 1, scale);
    let noise_b = create_noise(seed ^ 2, scale);
    let dist_entry_r = create_distance_entry(width, height, &mut rand);
    let dist_entry_g = create_distance_entry(width, height, &mut rand);
    let dist_entry_b = create_distance_entry(width, height, &mut rand);

    let image = image::ImageBuffer::from_fn(width, height, |i, j| {
        let x = i as f64;
        let y = j as f64;

        //let r = scale_float_to_u8(noise_r.get([x, y]));
        //let g = scale_float_to_u8(noise_g.get([x, y]));
        //let b = scale_float_to_u8(noise_b.get([x, y]));
        let r = scale_float_to_u8(dist_entry_r.scaled_distance(Vec2D::new(x, y)));
        let g = scale_float_to_u8(dist_entry_g.scaled_distance(Vec2D::new(x, y)));
        let b = scale_float_to_u8(dist_entry_b.scaled_distance(Vec2D::new(x, y)));
        image::Rgb([r, g, b])
    });
    image.save(opts.output_path).unwrap();
}

fn create_distance_entry(width: u32, height: u32, rand: &mut SmallRng) -> DistanceEntry {
    DistanceEntry::new(
        dist::DistanceType::Manhattan,
        Vec2D::new(width as f64, height as f64),
        Vec2D::new(rand.gen_range(0.0, width as f64), rand.gen_range(0.0, height as f64)),
        rand.gen(),
        rand.gen(),
    )
}

type Noise = ScalePoint<Fbm>;

fn create_noise(seed: u128, scale: f64) -> Noise {
    let fbm = Fbm::new().set_seed(seed as u32).set_persistence(0.25);
    ScalePoint::new(fbm).set_x_scale(scale).set_y_scale(scale)
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
