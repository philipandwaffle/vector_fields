use bevy::{
    ecs::{
        query::With,
        system::{Local, Query, Res, ResMut, Resource},
    },
    input::{mouse::MouseButton, Input},
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    utils::Instant,
    window::{PrimaryWindow, Window},
};

use crate::cam::MainCam;

use super::bindings::Bindings;

#[derive(Resource, Debug)]
pub struct ControlState {
    pub left_mouse_down: bool,
    pub left_mouse_up: bool,
    pub left_mouse_just_down: bool,
    pub double_click: bool,
    pub mouse_world_pos: Vec2,
}
impl Default for ControlState {
    fn default() -> Self {
        Self {
            left_mouse_down: false,
            left_mouse_up: false,
            left_mouse_just_down: false,
            double_click: false,
            mouse_world_pos: Vec2::ZERO,
        }
    }
}

pub struct DoubleClick {
    timer: Option<Instant>,
}
impl DoubleClick {
    pub fn is_double_click(&mut self, click_window: f32) -> bool {
        match self.timer {
            Some(t) => {
                let elapsed = t.elapsed().as_secs_f32();
                self.timer = Some(Instant::now());
                return elapsed <= click_window;
            }
            None => {
                self.timer = Some(Instant::now());
                return false;
            }
        }
    }
}
impl Default for DoubleClick {
    fn default() -> Self {
        Self { timer: None }
    }
}

pub fn update_control_state(
    mouse: Res<Input<MouseButton>>,
    mut control_state: ResMut<ControlState>,
    bindings: Res<Bindings>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &mut GlobalTransform), With<MainCam>>,
    mut double_click: Local<DoubleClick>,
) {
    control_state.left_mouse_down = mouse.pressed(MouseButton::Left);
    control_state.left_mouse_just_down = mouse.just_pressed(MouseButton::Left);
    if mouse.just_released(MouseButton::Left) {
        control_state.left_mouse_up = true;
        if double_click.is_double_click(bindings.double_click_window) {
            control_state.double_click = true;
        }
    }
    let (c, t) = camera.single();
    if let Some(world_mouse_pos) = windows
        .get_single()
        .unwrap()
        .cursor_position()
        .and_then(|cursor| c.viewport_to_world(t, cursor))
        .map(|ray| ray.origin.truncate())
    {
        control_state.mouse_world_pos = world_mouse_pos;
    }
}
