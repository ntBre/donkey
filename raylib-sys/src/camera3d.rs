use crate::{Camera3D, CameraPitch};

impl Camera3D {
    pub fn pitch(
        &mut self,
        angle: f32,
        lock_view: bool,
        rotate_around_target: bool,
        rotate_up: bool,
    ) {
        unsafe {
            CameraPitch(
                self,
                angle,
                lock_view,
                rotate_around_target,
                rotate_up,
            );
        }
    }
}
