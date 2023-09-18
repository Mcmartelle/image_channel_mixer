use image::imageops::{resize, FilterType};
use image::{self, DynamicImage, GenericImageView, RgbaImage};

pub fn nfloat_to_u8(val: f32) -> u8 {
    (val.clamp(0.0, 1.0) * 255.0 + 0.5) as u8
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
