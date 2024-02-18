use std::ffi::c_float;

use crate::{Matrix, Vector3};

#[rustfmt::skip]
pub fn MatrixZero() -> Matrix {
    Matrix {
        m0: 0.0, m4: 0.0, m8: 0.0, m12: 0.0,
        m1: 0.0, m5: 0.0, m9: 0.0, m13: 0.0,
        m2: 0.0, m6: 0.0, m10: 0.0, m14: 0.0,
        m3: 0.0, m7: 0.0, m11: 0.0, m15: 0.0,
    }
}

pub fn MatrixRotate(axis: Vector3, angle: c_float) -> Matrix {
    let mut result = MatrixZero();
    let Vector3 { mut x, mut y, mut z } = axis;
    let length_squared = x * x + y * y + z * z;
    if length_squared != 1.0 && length_squared != 0.0 {
        let ilength = 1.0 / length_squared.sqrt();
        x *= ilength;
        y *= ilength;
        z *= ilength;
    }

    let sinres = angle.sin();
    let cosres = angle.cos();
    let t = 1.0 - cosres;

    result.m0 = x * x * t + cosres;
    result.m1 = y * x * t + z * sinres;
    result.m2 = z * x * t - y * sinres;
    result.m3 = 0.0;

    result.m4 = x * y * t - z * sinres;
    result.m5 = y * y * t + cosres;
    result.m6 = z * y * t + x * sinres;
    result.m7 = 0.0;

    result.m8 = x * z * t + y * sinres;
    result.m9 = y * z * t - x * sinres;
    result.m10 = z * z * t + cosres;
    result.m11 = 0.0;

    result.m12 = 0.0;
    result.m13 = 0.0;
    result.m14 = 0.0;
    result.m15 = 1.0;

    result
}
