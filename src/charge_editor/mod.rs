use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use self::{
    charge_editor::{
        create_charge, edit_charge, edit_velocity, if_create_charge, if_edit_charge,
        if_edit_velocity, if_move_charge, move_charge, spawn_ui, update_editor_mode, EditorState,
    },
    icons::{drag_icons, setup_builders},
};

pub mod charge_editor;
mod icons;
pub mod ui_elements;

pub struct ChargeEditorPlugin;
impl Plugin for ChargeEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EditorState::new())
            .add_systems(Startup, (spawn_ui, setup_builders))
            .add_systems(
                Update,
                (
                    update_editor_mode,
                    create_charge.run_if(if_create_charge),
                    move_charge.run_if(if_move_charge),
                    edit_velocity.run_if(if_edit_velocity),
                    edit_charge.run_if(if_edit_charge),
                    drag_icons,
                )
                    .chain(),
            );
    }
}
