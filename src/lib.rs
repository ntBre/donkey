use std::ffi::CString;

use raylib_sys::InitWindow;

#[macro_export]
macro_rules! vector3 {
    ($x:expr, $y:expr, $z:expr) => {
        raylib_sys::Vector3 { x: $x, y: $y, z: $z }
    };
}

pub fn init_window(width: i32, height: i32, title: &str) {
    let title = CString::new(title).unwrap();
    unsafe {
        InitWindow(width, height, title.as_ptr());
    }
}

pub mod colors {
    use raylib_sys::Color;

    pub const LIGHTGRAY: Color = Color { r: 200, g: 200, b: 200, a: 255 };
    pub const GRAY: Color = Color { r: 130, g: 130, b: 130, a: 255 };
    pub const DARKGRAY: Color = Color { r: 80, g: 80, b: 80, a: 255 };
    pub const YELLOW: Color = Color { r: 253, g: 249, b: 0, a: 255 };
    pub const GOLD: Color = Color { r: 255, g: 203, b: 0, a: 255 };
    pub const ORANGE: Color = Color { r: 255, g: 161, b: 0, a: 255 };
    pub const PINK: Color = Color { r: 255, g: 109, b: 194, a: 255 };
    pub const RED: Color = Color { r: 230, g: 41, b: 55, a: 255 };
    pub const MAROON: Color = Color { r: 190, g: 33, b: 55, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 228, b: 48, a: 255 };
    pub const LIME: Color = Color { r: 0, g: 158, b: 47, a: 255 };
    pub const DARKGREEN: Color = Color { r: 0, g: 117, b: 44, a: 255 };
    pub const SKYBLUE: Color = Color { r: 102, g: 191, b: 255, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 121, b: 241, a: 255 };
    pub const DARKBLUE: Color = Color { r: 0, g: 82, b: 172, a: 255 };
    pub const PURPLE: Color = Color { r: 200, g: 122, b: 255, a: 255 };
    pub const VIOLET: Color = Color { r: 135, g: 60, b: 190, a: 255 };
    pub const DARKPURPLE: Color = Color { r: 112, g: 31, b: 126, a: 255 };
    pub const BEIGE: Color = Color { r: 211, g: 176, b: 131, a: 255 };
    pub const BROWN: Color = Color { r: 127, g: 106, b: 79, a: 255 };
    pub const DARKBROWN: Color = Color { r: 76, g: 63, b: 47, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const BLANK: Color = Color { r: 0, g: 0, b: 0, a: 0 };
    pub const MAGENTA: Color = Color { r: 255, g: 0, b: 255, a: 255 };
    pub const RAYWHITE: Color = Color { r: 245, g: 245, b: 245, a: 255 };

    pub trait IntoColor {
        fn into(self) -> Color;
    }

    pub fn color(c: impl IntoColor) -> Color {
        c.into()
    }

    impl IntoColor for u32 {
        fn into(self) -> Color {
            Color {
                r: 0xFF & (self >> 24) as u8,
                g: 0xFF & (self >> 16) as u8,
                b: 0xFF & (self >> 8) as u8,
                a: 0xFF & (self >> 0) as u8,
            }
        }
    }

    impl IntoColor for (u8, u8, u8, u8) {
        fn into(self) -> Color {
            let (r, g, b, a) = self;
            Color { r, g, b, a }
        }
    }

    impl IntoColor for [u8; 4] {
        fn into(self) -> Color {
            let [r, g, b, a] = self;
            Color { r, g, b, a }
        }
    }
}
