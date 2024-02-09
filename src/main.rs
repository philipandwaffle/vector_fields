use std::error::Error;

use bevy::{math::vec2, prelude::*, window::WindowMode};
use cam::CamPlugin;
use charge::{Charge, Charges};
use initializer::Initializer;
use json_parser::JSONParser;
use setting::Settings;
use system::{electric_field_system, SystemStatus};
use vector_field::VectorField;

mod cam;
mod charge;
mod initializer;
mod json_parser;
mod setting;
mod system;
mod utils;
mod vector_field;

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new();

    let settings = Settings::load()?;
    let resolution = settings.display.clone().as_resolution();
    let charges = JSONParser::load::<Charges>("assets/saves/charges.json")?;
    electric_field_system(&mut app);

    app.insert_resource(Msaa::Sample4)
        .insert_resource(settings)
        .insert_resource(VectorField::new([29, 19], 2))
        .insert_resource(charges)
        .insert_resource(SystemStatus::default())
        .add_systems(Startup, init_vector_field)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Particle Sim".into(),
                        resolution: resolution.into(),
                        // present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::BorderlessFullscreen,
                        // Tells wasm to resize the window according to the available canvas
                        fit_canvas_to_parent: true,
                        // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                        prevent_default_event_handling: false,
                        ..default()
                    }),
                    ..default()
                })
                // don't use linear sampling as image textures will be blurry
                .set(ImagePlugin::default_nearest()),
            CamPlugin,
        ));

    app.run();

    Ok(())
}

fn init_vector_field(
    mut commands: Commands,
    mut vector_field: ResMut<VectorField>,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    let vector = settings.simulation.vector.clone();
    let arrow_texture = asset_server.load(vector.texture);

    vector_field.init(&mut commands, arrow_texture, vector.spacing, vector.size);
}
