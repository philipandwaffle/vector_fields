use bevy::{
    math::{Quat, Vec2},
    render::color::Color,
};

pub fn dir_to_quat(dir: Vec2) -> Quat {
    // let x = if dir.x >= 0.0 { -1.0 } else { 1.0 };
    // return Quat::from_rotation_z(x * dir.y.acos());
    return Quat::from_rotation_z(f32::atan2(dir.y, dir.x));
}

pub fn mag_to_color(mag: f32) -> Color {
    return Color::hsla((f32::tanh(mag * 200.0) * 180.0) + 180.0, 0.5, 0.5, 1.0);
}
