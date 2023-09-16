use anyhow::Result;
use image::imageops::{resize, FilterType};
use image::io::Reader as ImageReader;

use image::{self, DynamicImage, GenericImageView, ImageBuffer, Pixel, RgbImage, Rgba, RgbaImage};
use octarine::Color;
use std::path::Path;

use crate::{Channel, Hsl};

use super::convert::u8_to_nfloat;

pub fn generate_hsl_image(args: &Hsl) -> Result<()> {
    let hue_raw = image::open(Path::new(&args.hue_file))?;
    let hue_img = resize_or_not(&hue_raw, args.width, args.height);
    let saturation_raw = ImageReader::open(&args.saturation_file)?.decode()?;
    let saturation_img = resize_or_not(&saturation_raw, args.width, args.height);
    let lightness_raw = ImageReader::open(&args.lightness_file)?.decode()?;
    let lightness_img = resize_or_not(&lightness_raw, args.width, args.height);
    let mut new_img: RgbImage = ImageBuffer::new(args.width.into(), args.height.into());
    for (x, y, pixel) in new_img.enumerate_pixels_mut() {
        let h: f32 = convert_channel(&args.hue_channel, hue_img.get_pixel(x, y).to_rgba());
        let s: f32 = convert_channel(
            &args.saturation_channel,
            saturation_img.get_pixel(x, y).to_rgba(),
        );
        let l: f32 = convert_channel(
            &args.lightness_channel,
            lightness_img.get_pixel(x, y).to_rgba(),
        );
        let (r, g, b) = Color::from_hsl(h, s, l).to_rgb();
        *pixel = image::Rgb([r, g, b]);
    }
    new_img.save(&args.output).unwrap();
    Ok(())
}

fn resize_or_not(img: &DynamicImage, width: u32, height: u32) -> RgbaImage {
    if img.dimensions() != (width, height) {
        return resize(img, width, height, FilterType::CatmullRom);
    } else {
        return img.to_rgba8();
    }
}

fn convert_channel(channel: &Channel, pixel: Rgba<u8>) -> f32 {
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

fn pixel_to_h(pixel: Rgba<u8>) -> f32 {
    Color::new(pixel.0[0], pixel.0[1], pixel.0[2])
        .get_hsl_hue()
        .clamp(0.0, 1.0)
}

fn pixel_to_s(pixel: Rgba<u8>) -> f32 {
    Color::new(pixel.0[0], pixel.0[1], pixel.0[2])
        .get_hsl_saturation()
        .clamp(0.0, 1.0)
}

fn pixel_to_l(pixel: Rgba<u8>) -> f32 {
    Color::new(pixel.0[0], pixel.0[1], pixel.0[2])
        .get_hsl_luminance()
        .clamp(0.0, 1.0)
}

fn pixel_to_r(pixel: Rgba<u8>) -> f32 {
    u8_to_nfloat(pixel.0[0])
}

fn pixel_to_g(pixel: Rgba<u8>) -> f32 {
    u8_to_nfloat(pixel.0[1])
}

fn pixel_to_b(pixel: Rgba<u8>) -> f32 {
    u8_to_nfloat(pixel.0[2])
}

fn pixel_to_a(pixel: Rgba<u8>) -> f32 {
    u8_to_nfloat(pixel.0[3])
}
