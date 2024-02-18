use crate::{
    Camera3D, CameraMoveForward, CameraMoveRight, CameraMoveUp, CameraPitch,
    CameraYaw,
};

impl Camera3D {
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
