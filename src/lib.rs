use std::ffi::CString;

use colors::IntoColor;
use keys::Key;
use raylib_sys::{
    BeginDrawing, BeginMode3D, Camera3D, ClearBackground, DrawCube,
    DrawCylinderEx, DrawSphere, EndDrawing, EndMode3D, GetFrameTime,
    InitWindow, IsKeyDown, TakeScreenshot, Vector3, WindowShouldClose,
};

pub mod colors;
pub mod keys;

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

    pub fn take_screenshot(&self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe {
            TakeScreenshot(filename.as_ptr());
        }
    }

    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { IsKeyDown(key as i32) }
    }

    /// TODO do one of those cool closure things here:
    /// ```
    /// win.draw(|canvas| {
    ///     canvas.clear_background(color);
    ///     ...
    /// })
    /// ```
    pub fn begin_drawing(&self) {
        unsafe { BeginDrawing() }
    }

    pub fn end_drawing(&self) {
        unsafe { EndDrawing() }
    }

    // drawing functions

    pub fn clear_background(&self, color: impl IntoColor) {
        unsafe { ClearBackground(color.into()) }
    }

    pub fn draw_cube(
        &self,
        center: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: impl IntoColor,
    ) {
        unsafe { DrawCube(center, width, height, length, color.into()) }
    }

    pub fn draw_sphere(
        &self,
        center: Vector3,
        radius: f32,
        color: impl IntoColor,
    ) {
        unsafe { DrawSphere(center, radius, color.into()) }
    }

    /// draw a cylinder from `start` to `end` with constant `radius`
    pub fn draw_cylinder(
        &self,
        start: Vector3,
        end: Vector3,
        radius: f32,
        color: impl IntoColor,
    ) {
        unsafe { DrawCylinderEx(start, end, radius, radius, 8, color.into()) }
    }

    // end drawing

    pub fn begin_mode3d(&self, camera: Camera3D) {
        unsafe { BeginMode3D(camera) }
    }

    pub fn end_mode3d(&self) {
        unsafe { EndMode3D() }
    }
}
