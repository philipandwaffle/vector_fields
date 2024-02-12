use std::{error::Error, time::Duration};

use bevy::{
    app::{App, PluginGroup, Startup, Update},
    asset::AssetServer,
    ecs::system::{Commands, Res, ResMut, Resource},
    render::{texture::ImagePlugin, view::Msaa},
    time::{Time, Timer, TimerMode},
    utils::default,
    window::{Window, WindowMode, WindowPlugin},
    DefaultPlugins,
};
use cam::CamPlugin;
use charge::Charges;
use charge_editor::{ui_elements::UIPlugin, ChargeEditorPlugin};
use controls::ControlPlugin;
use json_parser::JSONParser;
use setting::Settings;
use system::{electric_field_system, SystemStatus};
use vector_field::VectorField;

mod cam;
mod charge;
mod charge_editor;
mod controls;
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
    let vf_size = settings.simulation.field.size;
    let vf_res = settings.simulation.field.resolution;
    let charges = JSONParser::load::<Charges>("assets/saves/charges.json")?;
    electric_field_system(&mut app);

    app.insert_resource(Msaa::Sample4)
        .insert_resource(settings)
        .insert_resource(VectorField::new(vf_size, vf_res))
        .insert_resource(CurCharge::default())
        .insert_resource(charges)
        .insert_resource(SystemStatus::default())
        .add_systems(Startup, init_vector_field)
        .add_systems(Update, change_charge_list)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Particle Sim".into(),
                        resolution: resolution.into(),
                        // present_mode: PresentMode::AutoVsync,
                        mode: WindowMode::Windowed,
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
            ChargeEditorPlugin,
            UIPlugin,
            ControlPlugin,
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

#[derive(Resource, Default)]
struct CurCharge {
    t: Timer,
    i: usize,
}
fn change_charge_list(
    time: Res<Time>,
    mut cur_charge: ResMut<CurCharge>,
    mut charges: ResMut<Charges>,
) {
    cur_charge.t.tick(time.delta());

    if cur_charge.t.finished() {
        cur_charge.t = Timer::new(Duration::from_secs(20), TimerMode::Once);
        let path = format!("assets/saves/{}.json", cur_charge.i);
        charges.charges = JSONParser::load::<Charges>(&path).unwrap().charges;
        cur_charge.i += 1;
        if cur_charge.i == 6 {
            cur_charge.i = 0
        }
    }
}
