use bevy::math::{EulerRot, Quat};

pub const INCH_TO_M: f32 = 0.0254;

#[inline]
pub fn extent([x, y, z]: [f32; 3]) -> [f32; 3] {
    [x * INCH_TO_M, z * INCH_TO_M, y * INCH_TO_M]
}

#[inline]
pub fn position([x, y, z]: [f32; 3]) -> [f32; 3] {
    [x * INCH_TO_M, -z * INCH_TO_M, -y * INCH_TO_M]
}

#[inline]
pub fn direction([x, y, z]: [f32; 3]) -> [f32; 3] {
    [x, -z, -y]
}

#[inline]
pub fn rotation([x, y, z]: [f32; 3]) -> Quat {
    Quat::from_euler(EulerRot::XZY, x, -y, -z)
}
