use std::ffi::CString;

use raylib_sys::{LoadImage, UnloadImage};

use crate::DonkeyError;

pub struct Image {
    inner: raylib_sys::Image,
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            UnloadImage(self.inner);
        }
    }
}

impl Image {
    pub fn load(filename: impl Into<Vec<u8>>) -> Result<Self, DonkeyError> {
        unsafe {
            let s = CString::new(filename)?;
            let ret = LoadImage(s.as_ptr());
            if ret.data.is_null() {
                return Err(DonkeyError("failed to load image file"));
            }
            Ok(Self { inner: ret })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "failed to load image file")]
    fn test_load_image() {
        let got = Image::load("fake_file").unwrap();
        dbg!(got.inner.data.is_null());
    }
}
