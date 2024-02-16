//! 2D drawing primitives

use std::ffi::c_int;

use raylib_sys::DrawRectangle;

use crate::{colors::IntoColor, Window};

impl Window {
    /// draw a color-filled rectangle
    pub fn draw_rectangle(
        &self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: impl IntoColor,
    ) {
        unsafe {
            DrawRectangle(
                x as c_int,
                y as c_int,
                w as c_int,
                h as c_int,
                color.into(),
            );
        }
    }
}
