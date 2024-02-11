use bevy::app::{Plugin, Update};

use self::{bindings::Bindings, state::{update_control_state, ControlState}};

mod bindings;
mod state;

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Bindings::default())
            .insert_resource(ControlState::default())
            .add_systems(Update, update_control_state);
    }
}
