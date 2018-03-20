extern crate lodepng;
extern crate noise;
extern crate rgb;

use rgb::*;
use noise::{NoiseFn, OpenSimplex};
use std::path::Path;

fn main() {
    let path = Path::new("out.png");
    let noise = OpenSimplex::new();
    let buf = [255u8, 0, 0, 255, 240, 210, 123, 0, 125, 0, 0, 242];
    lodepng::encode_file(&path, &buf, 2, 2, lodepng::ColorType::RGB, 8).unwrap();
}
