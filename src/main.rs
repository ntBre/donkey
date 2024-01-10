use std::ffi::CString;

use raylib_sys::{
    BeginDrawing, BeginMode3D, Camera3D, CameraProjection_CAMERA_PERSPECTIVE,
    ClearBackground, DrawCube, EndDrawing, EndMode3D, GetColor, GetFrameTime,
    InitWindow, IsKeyDown, KeyboardKey_KEY_S, KeyboardKey_KEY_W,
    WindowShouldClose,
};

macro_rules! vector3 {
    ($x:expr, $y:expr, $z:expr) => {
        raylib_sys::Vector3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

/// basic example based on Tsoding's "Ridiculously Easy 3D in C" video:
/// https://www.youtube.com/watch?v=K7hWqxC_7Mw
fn main() {
    let width = 800;
    let height = 600;
    let title = CString::new("Example").unwrap();
    let cube_size = 1.0;
    let camera_speed = cube_size;
    let mut camera = Camera3D {
        position: vector3!(0.0, 0.0, -cube_size),
        target: vector3!(0.0, 0.0, 0.0),
        up: vector3!(0.0, 1.0, 0.0),
        fovy: 90.0,
        projection: CameraProjection_CAMERA_PERSPECTIVE as i32,
    };
    unsafe {
        InitWindow(width, height, title.as_ptr());
        let color = GetColor(0x181818AA);
        while !WindowShouldClose() {
            let dt = GetFrameTime();
            if IsKeyDown(KeyboardKey_KEY_W as i32) {
                camera.position.z += camera_speed * dt;
            }
            if IsKeyDown(KeyboardKey_KEY_S as i32) {
                camera.position.z -= camera_speed * dt;
            }
            BeginDrawing();
            ClearBackground(color);
            BeginMode3D(camera);
            DrawCube(
                vector3!(0.0, 0.0, 0.0),
                cube_size,
                cube_size,
                cube_size,
                GetColor(0xff0000ff),
            );
            EndMode3D();
            EndDrawing();
        }
    }
}
