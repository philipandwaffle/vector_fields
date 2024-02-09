use bevy::app::Plugin;

pub struct Initializer {
    config_path: String,
    save_path: String,
}
impl Plugin for Initializer {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.insert_resource(resource)
    }
}
