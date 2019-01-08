extern crate image;

use std::env;
use std::fs::{File, read_dir};
use std::path::{Path, PathBuf};

use image::{GenericImageView, GenericImage, RgbaImage, ImageBuffer, Pixel};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_dir = &args[1];
    let output_dir = Path::new(&args[2]);

    let mut input_files: Vec<PathBuf> = read_dir(input_dir).unwrap()
        .map(|path| path.unwrap().path()).collect();

    let length = input_files.len();
    let mut imgs_in = Vec::with_capacity(length);

    for path in input_files {
        imgs_in.push(image::open(path).unwrap());
    }

    let (width, height) = imgs_in[0].dimensions();

    for x in 0..width {
        let mut img_out: RgbaImage = ImageBuffer::new(length as u32, height);
        for t in 0..length {
            for y in 0..height {
                img_out.put_pixel(t as u32, y, imgs_in[t].get_pixel(x, y));
            }
        }
        let filename = output_dir.join(format!("{:0width$}.png", x + 1, width = (length as f32).log(10.0) as usize + 1));
        img_out.save(filename).unwrap();
    }

    println!("{}", imgs_in.len());
}
