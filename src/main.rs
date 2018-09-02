extern crate clap;
extern crate lodepng;
extern crate noise;
extern crate rgb;
extern crate time;

use rgb::*;
use noise::{Fbm, MultiFractal, NoiseFn, Seedable, ScalePoint};
use std::path::Path;
use std::string::String;
use clap::{App, Arg};

struct ArgResult {
    output_path: String,
    width: usize,
    height: usize,
    scale: f64,
}

fn parse_args() -> Result<ArgResult, String> {
    let matches = App::new("pcgbg")
        .arg(
            Arg::with_name("scale")
                .short("s")
                .long("scale")
                .value_name("SCALE")
                .takes_value(true)
                .help("Scale of noise"),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .value_name("WIDTH")
                .takes_value(true)
                .help("Width of image"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .value_name("HEIGHT")
                .takes_value(true)
                .help("Height of image"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .help("Name of file to output"),
        )
        .get_matches();
    let width = if let Some(width_str) = matches.value_of("width") {
        width_str.parse::<usize>().map_err(|e| e.to_string())?
    } else {
        500
    };
    let height = if let Some(height_str) = matches.value_of("height") {
        height_str.parse::<usize>().map_err(|e| e.to_string())?
    } else {
        500
    };
    let scale = if let Some(height_str) = matches.value_of("scale") {
        height_str.parse::<f64>().map_err(|e| e.to_string())?
    } else {
        0.005
    };
    Ok(ArgResult {
        output_path: matches.value_of("output").map(|s| s.to_owned()).unwrap_or("out.png".to_owned()),
        width,
        height,
        scale,
    })
}

fn main() {
    let args = parse_args().unwrap();
    let width = args.width;
    let height = args.height;
    let scale = args.scale;
    let path = Path::new(&args.output_path);
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
    lodepng::encode_file(&path, &buf, width, height, lodepng::ColorType::RGB, 8).unwrap();
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
    let time = time::get_time();
    (time.sec as u32) ^ (time.nsec as u32)
}
