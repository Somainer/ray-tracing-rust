use crate::color::{Color3d, write_color};
use std::io::Write;

pub struct PPMFile {
    height: usize,
    width: usize,
    spp: usize,
    pub buf: Vec<Color3d>
}

impl PPMFile {
    fn open_file(file_name: String) -> std::io::Result<std::fs::File> {
        let path = std::path::Path::new(file_name.as_str());
        std::fs::File::create(path)
    }

    #[inline]
    pub fn create(height: usize,
                  width: usize,
                  spp: usize,
                  buf: Vec<Color3d>) -> PPMFile {
        PPMFile {
            height,
            width,
            spp,
            buf
        }
    }

    // Take the ownership of file. After file is written, file is no more available.
    pub fn write_to(self, file_name: String) -> std::io::Result<()> {
        let mut fp = Self::open_file(file_name)?;

        fp.write_all(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        for c in self.buf {
            write_color(&mut fp, &c, self.spp)?
        }

        Ok(())
    }

    property! { height: usize }
    property! { width: usize }
}
