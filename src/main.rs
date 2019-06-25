use noise::{Fbm, MultiFractal, NoiseFn, ScalePoint, Seedable};
use std::path::PathBuf;
use structopt::StructOpt;

mod dist;

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
    let noise_r = create_noise(seed ^ 0, scale);
    let noise_g = create_noise(seed ^ 1, scale);
    let noise_b = create_noise(seed ^ 2, scale);
    let image = image::ImageBuffer::from_fn(width, height, |i, j| {
        let x = i as f64;
        let y = j as f64;

        let r = noise_output_to_u8(noise_r.get([x, y]));
        let g = noise_output_to_u8(noise_g.get([x, y]));
        let b = noise_output_to_u8(noise_b.get([x, y]));
        image::Rgb([r, g, b])
    });
    image.save(opts.output_path).unwrap();
}

type Noise = ScalePoint<Fbm>;

fn create_noise(seed: u32, scale: f64) -> Noise {
    let fbm = Fbm::new().set_seed(seed).set_persistence(0.25);
    ScalePoint::new(fbm).set_x_scale(scale).set_y_scale(scale)
}

fn noise_output_to_u8(val: f64) -> u8 {
    (val * std::u8::MAX as f64) as u8
}

fn get_time_seed() -> u32 {
    let start = std::time::SystemTime::now();
    let since_the_epoch = start
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_nanos() as u32
}
