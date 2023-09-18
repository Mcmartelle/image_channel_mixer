use anyhow::{anyhow, Result};
use image::imageops::{resize, FilterType};
use image::{self, DynamicImage, GenericImageView, RgbaImage};

pub fn nfloat_to_u8(val: f32) -> Result<u8> {
    if val + f32::EPSILON < 0.0 || val - f32::EPSILON > 1.0 {
        return Err(anyhow!("normalized float not between 0.0 and 1.0: {}", val));
    }
    Ok((val * 255.0 + 0.5) as u8)
}

pub fn u8_to_nfloat(val: u8) -> f32 {
    val as f32 / 255.0
}

pub fn resize_or_not(img: &DynamicImage, width: u32, height: u32) -> RgbaImage {
    if img.dimensions() != (width, height) {
        return resize(img, width, height, FilterType::CatmullRom);
    } else {
        return img.to_rgba8();
    }
}
