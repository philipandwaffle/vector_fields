use bevy::app::{App, Plugin, Startup, Update};

use self::charge_editor::{spawn_ui, update_editor_mode, EditorState};

pub mod charge_editor;
pub mod ui_elements;

pub struct ChargeEditorPlugin;
impl Plugin for ChargeEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorState::new())
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, update_editor_mode);
    }
}
