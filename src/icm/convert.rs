use anyhow::{anyhow, Result};

pub fn nfloat_to_u8(val: f32) -> Result<u8> {
    if val + f32::EPSILON < 0.0 || val - f32::EPSILON > 1.0 {
        return Err(anyhow!("normalized float not between 0.0 and 1.0: {}", val));
    }
    Ok((val * 255.0 + 0.5) as u8)
}

pub fn u8_to_nfloat(val: u8) -> f32 {
    val as f32 / 255.0
}
