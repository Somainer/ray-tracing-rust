use crate::vec3::Vec3;
use std::io;
use crate::util::clamp;

pub type Color3d = Vec3<f64>;

#[inline]
fn normalize_color(c: f64) -> u8 {
    (clamp(c, 0.0, 0.999) * 256.0) as u8
}

#[inline]
fn replace_nan(c: f64) -> f64 {
    if c != c { 0.0 } else { c }
}

#[inline]
fn correction(color: f64, scale: f64) -> f64 {
    (replace_nan(color) * scale).sqrt()
}

pub fn corrected_color(color: Color3d, spp: usize) -> [u8; 3] {
    let scale = 1.0 / spp as f64;
    [
        normalize_color(correction(color.x, scale)),
        normalize_color(correction(color.y, scale)),
        normalize_color(correction(color.z, scale)),
    ]
}

pub fn write_color(fp: &mut impl io::Write, color: &Color3d, samples_per_pixel: usize) -> io::Result<()> {
    // fp.write(format!("{} {} {}\n",
    //                  normalize_color(color.x),
    //                  normalize_color(color.y),
    //                  normalize_color(color.z)
    // ).as_bytes()).map(|_| ())
    fp.write(&corrected_color(*color, samples_per_pixel))?;
    Ok(())
}
