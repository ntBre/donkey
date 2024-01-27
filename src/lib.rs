use std::ffi::CString;

use keys::Key;
use raylib_sys::{
    BeginDrawing, EndDrawing, GetFrameTime, InitWindow, IsKeyDown,
    WindowShouldClose,
};

#[macro_export]
macro_rules! vector3 {
    ($x:expr, $y:expr, $z:expr) => {
        raylib_sys::Vector3 { x: $x, y: $y, z: $z }
    };
}

pub struct Window;

impl Window {
    pub fn init(width: i32, height: i32, title: &str) -> Self {
        let title = CString::new(title).unwrap();
        unsafe {
            InitWindow(width, height, title.as_ptr());
        }
        Self
    }

    pub fn should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn get_frame_time(&self) -> f32 {
        unsafe { GetFrameTime() }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { IsKeyDown(key as i32) }
    }

    pub fn begin_drawing(&self) {
        unsafe { BeginDrawing() }
    }

    pub fn end_drawing(&self) {
        unsafe { EndDrawing() }
    }
}

pub mod keys {
    #[repr(i32)]
    pub enum Key {
        A = raylib_sys::KeyboardKey_KEY_A as i32,
        B = raylib_sys::KeyboardKey_KEY_B as i32,
        C = raylib_sys::KeyboardKey_KEY_C as i32,
        D = raylib_sys::KeyboardKey_KEY_D as i32,
        E = raylib_sys::KeyboardKey_KEY_E as i32,
        F = raylib_sys::KeyboardKey_KEY_F as i32,
        G = raylib_sys::KeyboardKey_KEY_G as i32,
        H = raylib_sys::KeyboardKey_KEY_H as i32,
        I = raylib_sys::KeyboardKey_KEY_I as i32,
        J = raylib_sys::KeyboardKey_KEY_J as i32,
        K = raylib_sys::KeyboardKey_KEY_K as i32,
        L = raylib_sys::KeyboardKey_KEY_L as i32,
        M = raylib_sys::KeyboardKey_KEY_M as i32,
        N = raylib_sys::KeyboardKey_KEY_N as i32,
        O = raylib_sys::KeyboardKey_KEY_O as i32,
        P = raylib_sys::KeyboardKey_KEY_P as i32,
        Q = raylib_sys::KeyboardKey_KEY_Q as i32,
        R = raylib_sys::KeyboardKey_KEY_R as i32,
        S = raylib_sys::KeyboardKey_KEY_S as i32,
        T = raylib_sys::KeyboardKey_KEY_T as i32,
        U = raylib_sys::KeyboardKey_KEY_U as i32,
        V = raylib_sys::KeyboardKey_KEY_V as i32,
        W = raylib_sys::KeyboardKey_KEY_W as i32,
        X = raylib_sys::KeyboardKey_KEY_X as i32,
        Y = raylib_sys::KeyboardKey_KEY_Y as i32,
        Z = raylib_sys::KeyboardKey_KEY_Z as i32,
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
                r: (self >> 24) as u8,
                g: (self >> 16) as u8,
                b: (self >> 8) as u8,
                a: self as u8,
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
