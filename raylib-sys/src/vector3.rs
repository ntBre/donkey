use crate::{Matrix, Vector3};

pub fn Vector3Add(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x + v2.x,
        y: v1.y + v2.y,
        z: v1.z + v2.z,
    }
}

pub fn Vector3Subtract(v1: Vector3, v2: Vector3) -> Vector3 {
    Vector3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
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
