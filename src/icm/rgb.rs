use anyhow::Result;

use image::io::Reader as ImageReader;

use image::{self, ImageBuffer, Pixel, RgbImage, Rgba};
use octarine::Color;
use std::path::Path;

use crate::{Channel, Rgb};

use super::convert::{resize_or_not, nfloat_to_u8};

pub fn generate_rgb_image(args: &Rgb) -> Result<()> {
    let red_raw = image::open(Path::new(&args.red_file))?;
    let red_img = resize_or_not(&red_raw, args.width, args.height);
    let green_raw = ImageReader::open(&args.green_file)?.decode()?;
    let green_img = resize_or_not(&green_raw, args.width, args.height);
    let blue_raw = ImageReader::open(&args.blue_file)?.decode()?;
    let blue_img = resize_or_not(&blue_raw, args.width, args.height);
    let mut new_img: RgbImage = ImageBuffer::new(args.width.into(), args.height.into());
    for (x, y, pixel) in new_img.enumerate_pixels_mut() {
        let r: u8 = convert_channel(&args.red_channel, red_img.get_pixel(x, y).to_rgba());
        let g: u8 = convert_channel(
            &args.green_channel,
            green_img.get_pixel(x, y).to_rgba(),
        );
        let b: u8 = convert_channel(
            &args.blue_channel,
            blue_img.get_pixel(x, y).to_rgba(),
        );
        *pixel = image::Rgb([r, g, b]);
    }
    new_img.save(&args.output).unwrap();
    Ok(())
}

fn convert_channel(channel: &Channel, pixel: Rgba<u8>) -> u8 {
    match channel {
        Channel::H => pixel_to_h(pixel),
        Channel::S => pixel_to_s(pixel),
        Channel::L => pixel_to_l(pixel),
        Channel::R => pixel_to_r(pixel),
        Channel::G => pixel_to_g(pixel),
        Channel::B => pixel_to_b(pixel),
        Channel::A => pixel_to_a(pixel),
    }
}

fn pixel_to_h(pixel: Rgba<u8>) -> u8 {
    nfloat_to_u8(Color::new(pixel.0[0], pixel.0[1], pixel.0[2])
        .get_hsl_hue())
}

fn pixel_to_s(pixel: Rgba<u8>) -> u8 {
    nfloat_to_u8(Color::new(pixel.0[0], pixel.0[1], pixel.0[2])
        .get_hsl_saturation())
}

fn pixel_to_l(pixel: Rgba<u8>) -> u8 {
    nfloat_to_u8(Color::new(pixel.0[0], pixel.0[1], pixel.0[2])
        .get_hsl_luminance())
}

fn pixel_to_r(pixel: Rgba<u8>) -> u8 {
   pixel.0[0]
}

fn pixel_to_g(pixel: Rgba<u8>) -> u8 {
    pixel.0[1]
}

fn pixel_to_b(pixel: Rgba<u8>) -> u8 {
    pixel.0[2]
}

fn pixel_to_a(pixel: Rgba<u8>) -> u8 {
    pixel.0[3]
}
