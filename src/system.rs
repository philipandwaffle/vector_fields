use std::vec;

use bevy::{
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
    electric_field: bool,
}
impl Default for SystemStatus {
    fn default() -> Self {
        Self {
            electric_field: true,
        }
    }
}

pub fn electric_field_system() -> NodeConfigs<Box<dyn System<In = (), Out = ()>>> {
    return (update_field, update_arrows, move_charges)
        .chain()
        .run_if(run_electric_field);
}
fn run_electric_field(status: Res<SystemStatus>, vector_field: Option<Res<VectorField>>) -> bool {
    return status.electric_field && vector_field.is_some();
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
