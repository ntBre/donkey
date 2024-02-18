use std::{
    error::Error,
    ffi::{c_int, CString, NulError},
    fmt::Display,
};

use colors::IntoColor;
use keys::Key;
use raylib_sys::{matrix::MatrixRotate, GetScreenHeight};
use raylib_sys::{
    BeginDrawing, BeginMode3D, Camera3D, CameraMoveForward, CameraMoveRight,
    CameraMoveToTarget, CameraMoveUp, CameraPitch, CameraRoll, CameraYaw,
    ClearBackground, DrawCube, DrawCylinderEx, DrawSphere, DrawText,
    EndDrawing, EndMode3D, GamepadAxis_GAMEPAD_AXIS_LEFT_X,
    GamepadAxis_GAMEPAD_AXIS_LEFT_Y, GamepadAxis_GAMEPAD_AXIS_RIGHT_X,
    GamepadAxis_GAMEPAD_AXIS_RIGHT_Y, GetCameraUp, GetFrameTime,
    GetGamepadAxisMovement, GetMouseDelta, GetMouseWheelMove, InitWindow,
    IsGamepadAvailable, IsKeyDown, IsKeyPressed, IsMouseButtonDown,
    KeyboardKey_KEY_KP_ADD, KeyboardKey_KEY_KP_SUBTRACT, MeasureText,
    MouseButton_MOUSE_BUTTON_LEFT, MouseButton_MOUSE_BUTTON_MIDDLE,
    SetTargetFPS, TakeScreenshot, WindowShouldClose,
    CAMERA_MOUSE_MOVE_SENSITIVITY, CAMERA_MOVE_SPEED, CAMERA_ORBITAL_SPEED,
    CAMERA_PAN_SPEED, CAMERA_ROTATION_SPEED,
};

pub use raylib_sys::{Rectangle, Vector2, Vector3};

pub mod colors;
pub mod image;
pub mod keys;
pub mod twod;

#[derive(Debug)]
pub struct DonkeyError(&'static str);

impl Error for DonkeyError {}

impl Display for DonkeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DonkeyError({})", self.0)
    }
}

impl From<NulError> for DonkeyError {
    fn from(_value: NulError) -> Self {
        Self("NulError")
    }
}

#[derive(PartialEq)]
#[repr(i32)]
pub enum CameraMode {
    /// Camera custom, controlled by user (UpdateCamera() does nothing)
    Custom = 0,
    /// Camera free mode
    Free,
    /// Camera orbital, around target, zoom supported
    Orbital,
    /// Camera first person
    FirstPerson,
    /// Camera third person
    ThirdPerson,
}

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

    pub fn set_target_fps(&self, target: usize) {
        unsafe { SetTargetFPS(target as i32) }
    }

    pub fn should_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn get_frame_time(&self) -> f32 {
        unsafe { GetFrameTime() }
    }

    pub fn get_screen_height(&self) -> c_int {
        unsafe { GetScreenHeight() }
    }

    pub fn take_screenshot(&self, filename: &str) {
        let filename = CString::new(filename).unwrap();
        unsafe {
            TakeScreenshot(filename.as_ptr());
        }
    }

    /// Check if a key is being pressed
    pub fn is_key_down(&self, key: Key) -> bool {
        unsafe { IsKeyDown(key as i32) }
    }

    /// Check if a key has been pressed once
    pub fn is_key_pressed(&self, key: Key) -> bool {
        unsafe { IsKeyPressed(key as i32) }
    }

    /// TODO do one of those cool closure things here:
    /// ```notrust
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

    /// I didn't like the default UpdateCamera from rcamera, so this is a
    /// modification to require holding down the mouse button to rotate
    pub fn update_camera(&self, camera: &mut Camera3D, mode: CameraMode) {
        unsafe {
            use CameraMode as CM;
            let mouse_position_delta = GetMouseDelta();
            let move_in_world_plane =
                matches!(mode, CM::FirstPerson | CM::ThirdPerson);
            let rotate_around_target =
                matches!(mode, CM::ThirdPerson | CM::Orbital);
            let lock_view =
                matches!(mode, CM::FirstPerson | CM::ThirdPerson | CM::Orbital);
            let rotate_up = false;

            if mode == CM::Orbital {
                // Orbital can just orbit
                let rotation = MatrixRotate(
                    GetCameraUp(camera),
                    CAMERA_ORBITAL_SPEED * GetFrameTime(),
                );
                let mut view = camera.position - camera.target;
                view = view.transform(rotation);
                camera.position = camera.target + view;
            } else {
                // Camera rotation
                if self.is_key_down(Key::Down) {
                    CameraPitch(
                        camera,
                        -CAMERA_ROTATION_SPEED,
                        lock_view,
                        rotate_around_target,
                        rotate_up,
                    )
                };
                if self.is_key_down(Key::Up) {
                    CameraPitch(
                        camera,
                        CAMERA_ROTATION_SPEED,
                        lock_view,
                        rotate_around_target,
                        rotate_up,
                    )
                };
                if self.is_key_down(Key::Right) {
                    CameraYaw(
                        camera,
                        -CAMERA_ROTATION_SPEED,
                        rotate_around_target,
                    )
                };
                if self.is_key_down(Key::Left) {
                    CameraYaw(
                        camera,
                        CAMERA_ROTATION_SPEED,
                        rotate_around_target,
                    )
                };
                if self.is_key_down(Key::Q) {
                    CameraRoll(camera, -CAMERA_ROTATION_SPEED)
                };
                if self.is_key_down(Key::E) {
                    CameraRoll(camera, CAMERA_ROTATION_SPEED)
                };

                // Camera movement
                if !IsGamepadAvailable(0) {
                    // Camera pan (for CAMERA_FREE)
                    if (mode == CM::Free)
                        && (IsMouseButtonDown(
                            MouseButton_MOUSE_BUTTON_MIDDLE as i32,
                        ))
                    {
                        let mouse_delta = GetMouseDelta();
                        if mouse_delta.x > 0.0 {
                            CameraMoveRight(
                                camera,
                                CAMERA_PAN_SPEED,
                                move_in_world_plane,
                            );
                        }
                        if mouse_delta.x < 0.0 {
                            CameraMoveRight(
                                camera,
                                -CAMERA_PAN_SPEED,
                                move_in_world_plane,
                            );
                        }
                        if mouse_delta.y > 0.0 {
                            CameraMoveUp(camera, -CAMERA_PAN_SPEED);
                        }
                        if mouse_delta.y < 0.0 {
                            CameraMoveUp(camera, CAMERA_PAN_SPEED);
                        }
                    } else {
                        // Mouse support
                        if IsMouseButtonDown(
                            MouseButton_MOUSE_BUTTON_LEFT as i32,
                        ) {
                            CameraYaw(
                                camera,
                                -mouse_position_delta.x
                                    * CAMERA_MOUSE_MOVE_SENSITIVITY,
                                rotate_around_target,
                            );
                            CameraPitch(
                                camera,
                                -mouse_position_delta.y
                                    * CAMERA_MOUSE_MOVE_SENSITIVITY,
                                lock_view,
                                rotate_around_target,
                                rotate_up,
                            );
                        }
                    }

                    // Keyboard support
                    if self.is_key_down(Key::W) {
                        CameraMoveForward(
                            camera,
                            CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                    if self.is_key_down(Key::A) {
                        CameraMoveRight(
                            camera,
                            -CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                    if self.is_key_down(Key::S) {
                        CameraMoveForward(
                            camera,
                            -CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                    if self.is_key_down(Key::D) {
                        CameraMoveRight(
                            camera,
                            CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                } else {
                    // Gamepad controller support
                    CameraYaw(
                        camera,
                        -(GetGamepadAxisMovement(
                            0,
                            GamepadAxis_GAMEPAD_AXIS_RIGHT_X as i32,
                        ) * 2.0)
                            * CAMERA_MOUSE_MOVE_SENSITIVITY,
                        rotate_around_target,
                    );
                    CameraPitch(
                        camera,
                        -(GetGamepadAxisMovement(
                            0,
                            GamepadAxis_GAMEPAD_AXIS_RIGHT_Y as i32,
                        ) * 2.0)
                            * CAMERA_MOUSE_MOVE_SENSITIVITY,
                        lock_view,
                        rotate_around_target,
                        rotate_up,
                    );

                    if GetGamepadAxisMovement(
                        0,
                        GamepadAxis_GAMEPAD_AXIS_LEFT_Y as i32,
                    ) <= -0.25
                    {
                        CameraMoveForward(
                            camera,
                            CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                    if GetGamepadAxisMovement(
                        0,
                        GamepadAxis_GAMEPAD_AXIS_LEFT_X as i32,
                    ) <= -0.25
                    {
                        CameraMoveRight(
                            camera,
                            -CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                    if GetGamepadAxisMovement(
                        0,
                        GamepadAxis_GAMEPAD_AXIS_LEFT_Y as i32,
                    ) >= 0.25
                    {
                        CameraMoveForward(
                            camera,
                            -CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                    if GetGamepadAxisMovement(
                        0,
                        GamepadAxis_GAMEPAD_AXIS_LEFT_X as i32,
                    ) >= 0.25
                    {
                        CameraMoveRight(
                            camera,
                            CAMERA_MOVE_SPEED,
                            move_in_world_plane,
                        );
                    }
                }

                if mode == CM::Free {
                    if self.is_key_down(Key::Space) {
                        CameraMoveUp(camera, CAMERA_MOVE_SPEED);
                    }
                    if self.is_key_down(Key::LControl) {
                        CameraMoveUp(camera, -CAMERA_MOVE_SPEED);
                    }
                }
            }

            if matches!(mode, CM::ThirdPerson | CM::Orbital | CM::Free) {
                // Zoom target distance
                CameraMoveToTarget(camera, -GetMouseWheelMove());
                if IsKeyPressed(KeyboardKey_KEY_KP_SUBTRACT as i32) {
                    CameraMoveToTarget(camera, 2.0);
                }
                if IsKeyPressed(KeyboardKey_KEY_KP_ADD as i32) {
                    CameraMoveToTarget(camera, -2.0);
                }
            }
        }
    }

    pub fn end_mode3d(&self) {
        unsafe { EndMode3D() }
    }

    pub fn draw_text(
        &self,
        text: impl Into<Vec<u8>>,
        x: usize,
        y: usize,
        font_size: usize,
        color: impl IntoColor,
    ) {
        unsafe {
            let cstr = CString::new(text).unwrap();
            DrawText(
                cstr.as_ptr(),
                x as c_int,
                y as c_int,
                font_size as c_int,
                color.into(),
            );
        }
    }

    /// Measure the width of `text` for the default font with `font_size`.
    /// panics if `text` cannot be converted to a [CString]
    pub fn measure_text(&self, text: &str, font_size: usize) -> c_int {
        unsafe {
            let s = CString::new(text).unwrap();
            MeasureText(s.as_ptr(), font_size as c_int)
        }
    }
}
