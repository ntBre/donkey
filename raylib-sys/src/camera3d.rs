use crate::{
    Camera3D, CameraMoveForward, CameraMoveRight, CameraMoveUp, CameraPitch,
    CameraProjection_CAMERA_ORTHOGRAPHIC, CameraProjection_CAMERA_PERSPECTIVE,
    CameraYaw, Vector3,
};

#[repr(i32)]
pub enum CameraProjection {
    Perspective = CameraProjection_CAMERA_PERSPECTIVE as i32,
    Orthographic = CameraProjection_CAMERA_ORTHOGRAPHIC as i32,
}

impl Camera3D {
    pub fn new(
        position: Vector3,
        target: Vector3,
        up: Vector3,
        fovy: f32,
        projection: CameraProjection,
    ) -> Self {
        Self {
            position,
            target,
            up,
            fovy,
            projection: projection as i32,
        }
    }

    pub fn pitch(
        &mut self,
        angle: f32,
        lock_view: bool,
        rotate_around_target: bool,
        rotate_up: bool,
    ) {
        unsafe {
            CameraPitch(self, angle, lock_view, rotate_around_target, rotate_up)
        }
    }

    pub fn yaw(&mut self, angle: f32, rotateAroundTarget: bool) {
        unsafe { CameraYaw(self, angle, rotateAroundTarget) }
    }

    pub fn move_right(&mut self, distance: f32, moveInWorldPlane: bool) {
        unsafe { CameraMoveRight(self, distance, moveInWorldPlane) }
    }

    pub fn move_up(&mut self, distance: f32) {
        unsafe { CameraMoveUp(self, distance) }
    }

    pub fn move_forward(&mut self, distance: f32, moveInWorldPlane: bool) {
        unsafe { CameraMoveForward(self, distance, moveInWorldPlane) }
    }
}
