use image::{GenericImage, Pixel, Rgb, DynamicImage, GenericImageView, ImageError};
use image::io::Reader as ImageReader;
use crate::texture::Texture;
use crate::color::Color3d;
use crate::vec3::Point3d;
use crate::util::clamp;

#[derive(Clone)]
pub struct ImageTexture<Image: GenericImage + Send + Sync> {
    image: Image
}

impl Texture for ImageTexture<DynamicImage> {
    fn eval(&self, mut u: f64, mut v: f64, _p: Point3d) -> Color3d {
        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);

        let i =
            clamp((u * self.width() as f64) as usize, 0, self.width() - 1);
        let j =
            clamp((v * self.height() as f64) as usize, 0, self.height() - 1);

        let pixel = self.image.get_pixel(i as u32, j as u32);
        let Rgb([r, g, b]) = pixel.to_rgb();

        Color3d::new(
            r as f64 / 255.0,
            g as f64 / 255.0,
            b as f64 / 255.0,
        )
    }
}

impl<Image: GenericImage + Send + Sync> ImageTexture<Image> {
    #[inline]
    pub fn width(&self) -> usize {
        self.image.width() as usize
    }

    #[inline]
    pub fn height(&self) -> usize {
        self.image.height() as usize
    }

    pub fn new(image: Image) -> Self {
        Self { image }
    }
}

impl ImageTexture<DynamicImage> {

    pub fn from_file(file_name: String) -> Result<Self, ImageError> {
        let image = ImageReader::open(file_name)?.decode()?;

        Ok(Self::new(image))
    }
}
