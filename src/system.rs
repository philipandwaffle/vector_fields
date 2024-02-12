use std::vec;

use bevy::{
    app::{App, Update},
    ecs::{
        schedule::{IntoSystemConfigs, NodeConfigs},
        system::{Query, Res, ResMut, Resource, System},
    },
    sprite::Sprite,
    transform::components::Transform,
};

use crate::{charge::Charges, setting::Settings, vector_field::VectorField};

#[derive(Resource)]
pub struct SystemStatus {
    update_field: bool,
    move_charges: bool,
}
impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            update_field: true,
            move_charges: false,
        }
    }
}

pub fn electric_field_system(app: &mut App) {
    app.add_systems(
        Update,
        (update_field, update_arrows)
            .chain()
            .run_if(if_update_field),
    );
    app.add_systems(Update, move_charges.run_if(if_move_charges));
}

fn if_update_field(
    status: Res<SystemStatus>,
    vector_field: Option<Res<VectorField>>,
    charges: Option<Res<Charges>>,
) -> bool {
    return status.update_field && vector_field.is_some() && charges.is_some();
}
fn update_field(mut vector_field: ResMut<VectorField>, charges: Res<Charges>) {
    charges.apply_to_field(&mut vector_field);
}
fn update_arrows(
    vector_field: Res<VectorField>,
    mut sprite_query: Query<(&mut Sprite, &mut Transform)>,
) {
    if let Err(e) = vector_field.update_sprites(&mut sprite_query) {
        print!("Error updating vector field sprites {}", e);
    }
}

fn if_move_charges(status: Res<SystemStatus>, charges: Option<Res<Charges>>) -> bool {
    return status.move_charges && charges.is_some();
}
fn move_charges(
    mut charges: ResMut<Charges>,
    vector_field: Res<VectorField>,
    settings: Res<Settings>,
) {
    let time_scale = settings.simulation.time_scale;

    let [width, height] = vector_field.get_shape();
    let bl = vector_field.coords[0][0];
    let tr = vector_field.coords[height - 1][width - 1];
    charges.update_velocities(time_scale);
    charges.move_charges(time_scale, [bl.x, tr.x, bl.y, tr.y]);
}
