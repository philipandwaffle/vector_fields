use bevy::{
    app::{Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        system::{Commands, Query},
    },
    math::vec3,
    prelude::default,
    transform::components::Transform,
};

#[derive(Component)]
pub struct MainCam;
pub struct CamPlugin;
impl Plugin for CamPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup_cam);
        // app.add_systems(Update, move_cam);
    }
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(vec3(0.0, 0.0, 0.0)),
            ..default()
        },
        MainCam,
    ));
}

fn move_cam(mut main_cam: Query<(&MainCam, &mut Transform)>) {
    let cam_res = main_cam.get_single_mut();
}
