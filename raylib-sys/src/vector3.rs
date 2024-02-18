use std::ops::{Add, Sub};

use crate::{Matrix, Vector3};

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub fn Vector3Add(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3Add(self, rhs)
    }
}

pub fn Vector3Subtract(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3Subtract(self, rhs)
    }
}

pub fn Vector3Transform(v: Vector3, mat: Matrix) -> Vector3 {
    let Vector3 { x, y, z } = v;
    Vector3 {
        x: mat.m0 * x + mat.m4 * y + mat.m8 * z + mat.m12,
        y: mat.m1 * x + mat.m5 * y + mat.m9 * z + mat.m13,
        z: mat.m2 * x + mat.m6 * y + mat.m10 * z + mat.m14,
    }
}
