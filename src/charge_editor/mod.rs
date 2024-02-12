use bevy::app::{App, Plugin, Startup, Update};

use self::{
    charge_editor::{handle_input, spawn_ui, update_editor_mode, EditorState},
    icons::setup_builders,
};

pub mod charge_editor;
mod icons;
pub mod ui_elements;

pub struct ChargeEditorPlugin;
impl Plugin for ChargeEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorState::new())
            .add_systems(Startup, (spawn_ui, setup_builders))
            .add_systems(Update, (update_editor_mode, handle_input));
    }
}
