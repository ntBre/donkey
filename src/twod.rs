//! 2D drawing primitives

use std::ffi::c_int;

use raylib_sys::{DrawRectangle, DrawRectangleV};

use crate::{colors::IntoColor, Window};

pub use raylib_sys::Vector2;

#[macro_export]
macro_rules! vector2 {
    ($x:expr, $y:expr) => {
        $crate::twod::Vector2 { x: $x, y: $y }
    };
}

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

    /// draw a color-filled rectangle with vector `position` and `size`
    pub fn draw_rectangle_v(
        &self,
        position: Vector2,
        size: Vector2,
        color: impl IntoColor,
    ) {
        unsafe {
            DrawRectangleV(position, size, color.into());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_vector2() {
        let v1 = vector2!(1.0, 2.0);
        let v2 = Vector2 { x: 1.0, y: 2.0 };
        assert_eq!(v1.x, v2.x);
        assert_eq!(v1.y, v2.y);
    }
}
