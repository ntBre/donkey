use donkey::{
    colors::color, keys::Key, vector3, Camera3D, CameraMode, CameraProjection,
    Window,
};

/// basic example based on Tsoding's "Ridiculously Easy 3D in C" video:
/// https://www.youtube.com/watch?v=K7hWqxC_7Mw
fn main() {
    let width = 800;
    let height = 600;
    let title = "review";
    let cube_size = 1.0;
    let camera_speed = cube_size;
    let mut camera = Camera3D::new(
        vector3!(0.0, 0.0, -cube_size),
        vector3!(0.0, 0.0, 0.0),
        vector3!(0.0, 1.0, 0.0),
        90.0,
        CameraProjection::Perspective,
    );
    let win = Window::init(width, height, title);
    let background = color(0x181818AA);
    while !win.should_close() {
        let dt = win.get_frame_time();
        if win.is_key_down(Key::W) {
            camera.position.z += camera_speed * dt;
        }
        if win.is_key_down(Key::S) {
            camera.position.z -= camera_speed * dt;
        }
        if win.is_key_down(Key::D) {
            camera.position.x += camera_speed * dt;
        }
        if win.is_key_down(Key::A) {
            camera.position.x -= camera_speed * dt;
        }
        win.begin_drawing();
        win.clear_background(background);
        win.begin_mode3d(camera);
        win.update_camera(&mut camera, CameraMode::ThirdPerson);
        win.draw_cube(
            vector3!(0.0, 0.0, 0.0),
            cube_size,
            cube_size,
            cube_size,
            0xff0000ff,
        );
        win.end_mode3d();
        win.end_drawing();
    }
}
