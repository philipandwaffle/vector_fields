use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct Bindings {
    pub double_click_window: f32,
}
impl Default for Bindings {
    fn default() -> Self {
        Self {
            double_click_window: 0.2,
        }
    }
}
