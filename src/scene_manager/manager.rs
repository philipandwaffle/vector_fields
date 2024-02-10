use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    hierarchy::DespawnRecursiveExt,
};

#[derive(Component)]
struct DespawnTag;

struct Manager {}
impl Manager {
    pub fn despawn(commands: &mut Commands) {
        // commands.
    }
}

fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}
