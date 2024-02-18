#![allow(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    improper_ctypes
)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const CAMERA_ORBITAL_SPEED: f32 = 0.5;
pub const CAMERA_ROTATION_SPEED: f32 = 0.03;
pub const CAMERA_PAN_SPEED: f32 = 0.2;
pub const CAMERA_MOUSE_MOVE_SENSITIVITY: f32 = 0.003;
pub const CAMERA_MOVE_SPEED: f32 = 0.09;

pub mod camera3d;
pub mod matrix;
pub mod rectangle;
pub mod vector2;
pub mod vector3;
