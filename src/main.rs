use noise::{Fbm, MultiFractal, NoiseFn, ScalePoint, Seedable};
use rgb::*;
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
    let noise_r = create_noise(seed ^ 0, scale);
    let noise_g = create_noise(seed ^ 1, scale);
    let noise_b = create_noise(seed ^ 2, scale);
    let mut buf = Vec::new();
    buf.resize(width * height, RGB8 { r: 0, g: 0, b: 0 });
    for j in 0..height {
        for i in 0..width {
            let x = i as f64;
            let y = j as f64;

            let mut pixel = &mut buf[i + j * width];
            pixel.r = noise_output_to_u8(noise_r.get([x, y]));
            pixel.g = noise_output_to_u8(noise_g.get([x, y]));
            pixel.b = noise_output_to_u8(noise_b.get([x, y]));
        }
    }
    lodepng::encode_file(
        opts.output_path.as_path(),
        &buf,
        width,
        height,
        lodepng::ColorType::RGB,
        8,
    )
    .unwrap();
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
