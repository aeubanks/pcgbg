mod pcgbg_buf;
mod pcgbg_dist;
mod pcgbg_noise;

use image::RgbImage;
use pcgbg_buf::Buf;
use pcgbg_dist::DistanceEntryDistribution;
use pcgbg_noise::NoiseDistribution;
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
    #[structopt(help = "Seed for RNG", long)]
    seed: Option<u64>,
}

fn main() {
    let opts = Opts::from_args();
    let width = opts.width;
    let height = opts.height;
    let scale = opts.scale;

    let mut rng = SmallRng::seed_from_u64(opts.seed.unwrap_or_else(|| get_time_seed() as u64));

    let noise_distribution = NoiseDistribution { scale };
    let noise_r = noise_distribution.sample(&mut rng);
    let noise_g = noise_distribution.sample(&mut rng);
    let noise_b = noise_distribution.sample(&mut rng);

    let dist_entry_distribution = DistanceEntryDistribution { width, height };
    let dist_entry_r = dist_entry_distribution.sample(&mut rng);
    let dist_entry_g = dist_entry_distribution.sample(&mut rng);
    let dist_entry_b = dist_entry_distribution.sample(&mut rng);

    let mut buf = Buf::new(width, height);
    buf.add(&noise_r, &[0.1, 0.0, 0.0]);
    buf.add(&noise_g, &[0.0, 0.1, 0.0]);
    buf.add(&noise_b, &[0.0, 0.0, 0.1]);
    buf.add(&dist_entry_r, &[1.0, 0.0, 0.0]);
    buf.add(&dist_entry_g, &[0.0, 1.0, 0.0]);
    buf.add(&dist_entry_b, &[0.0, 0.0, 1.0]);
    buf.normalize();

    let image = buf_to_image(&buf);
    image.save(opts.output_path).unwrap();
}

fn buf_to_image(buf: &Buf) -> RgbImage {
    RgbImage::from_fn(buf.width as u32, buf.height as u32, |i, j| {
        let x = i as usize;
        let y = j as usize;

        let r = scale_float_to_u8(buf.get(x, y, 0));
        let g = scale_float_to_u8(buf.get(x, y, 1));
        let b = scale_float_to_u8(buf.get(x, y, 2));
        image::Rgb([r, g, b])
    })
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
