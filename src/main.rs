use image::RgbImage;
#[cfg(test)]
use more_asserts::assert_le;
use pcgbg::pcgbg_buf::Buf;
use pcgbg::pcgbg_dist::{DistanceEntry, DistanceEntryDistribution};
use pcgbg::pcgbg_noise::NoiseDistribution;
use rand::distributions::Distribution;
use rand::rngs::SmallRng;
use rand::Rng;
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
    #[structopt(help = "Noise scale", long, default_value = "0.001")]
    scale: f64,
    #[structopt(help = "Seed for RNG", long)]
    seed: Option<u64>,
}

fn main() {
    let opts = Opts::from_args();
    let width = opts.width;
    let height = opts.height;
    let scale = opts.scale;

    let mut rng = SmallRng::seed_from_u64(opts.seed.unwrap_or_else(|| rand::thread_rng().gen()));

    let noise_distribution = NoiseDistribution { scale };
    let noise_r = noise_distribution.sample(&mut rng);
    let noise_g = noise_distribution.sample(&mut rng);
    let noise_b = noise_distribution.sample(&mut rng);

    let mut dist_entries = Vec::<DistanceEntry>::new();
    let dist_entry_distribution = DistanceEntryDistribution { width, height };
    for _ in 0..6 {
        dist_entries.push(dist_entry_distribution.sample(&mut rng));
    }

    let mut buf = Buf::new(width, height);
    for (i, dist_entry) in dist_entries.iter_mut().enumerate() {
        let mut color_ratios = vec![0.0; 3];
        let idx = i % color_ratios.len();
        color_ratios[idx] = rng.gen_range(0.1, 1.0);
        buf.add(dist_entry, &color_ratios);
    }
    buf.add(&noise_r, &[0.1, 0.0, 0.0]);
    buf.add(&noise_g, &[0.0, 0.1, 0.0]);
    buf.add(&noise_b, &[0.0, 0.0, 0.1]);
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
    (val * f64::from(std::u8::MAX)) as u8
}

#[test]
fn integration_test() {
    let width = 10;
    let height = 10;
    let scale = 0.5;
    let mut rng = SmallRng::seed_from_u64(1);

    let noise_distribution = NoiseDistribution { scale };
    let noise = noise_distribution.sample(&mut rng);
    let dist_entry_distribution = DistanceEntryDistribution { width, height };
    let dist_entry = dist_entry_distribution.sample(&mut rng);

    let mut buf = Buf::new(width, height);
    buf.add(&noise, &[1.4, 0.1, 0.0]);
    buf.add(&dist_entry, &[0.0, 0.1, 0.5]);
    buf.normalize();

    for y in 0..height {
        for x in 0..width {
            for c in 0..3 {
                let val = buf.get(x, y, c);
                assert_le!(0.0, val);
                assert_le!(val, 1.0);
            }
        }
    }

    let _image = buf_to_image(&buf);
}
