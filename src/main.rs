use bevy::{math::vec2, prelude::*, window::WindowMode};
use cam::CamPlugin;
use charge::{Charge, Charges};
use vector_field::VectorField;

mod cam;
mod charge;
mod utils;
mod vector_field;

fn main() {
    let mut app = App::new();

    // let a = Charges::new(vec![
    //     Charge::new(-1.0, vec2(-100.0, 0.0), vec2(0.0, 0.5)),
    //     Charge::new(1.0, vec2(0.0, 100.0), vec2(0.5, 0.0)),
    //     Charge::new(1.0, vec2(100.0, 0.0), vec2(0.0, -0.5)),
    //     Charge::new(-1.0, vec2(0.0, -100.0), vec2(-0.5, 0.5)),
    // ]);

    let a = Charges::new(vec![
        Charge::new(-10.0, vec2(-100.0, 0.0), vec2(0.0, 0.6)),
        Charge::new(10.0, vec2(100.0, 0.0), vec2(0.0, -0.6)),
    ]);

    app.insert_resource(Msaa::Sample4)
        .insert_resource(VectorField::new([29, 19], 2))
        .insert_resource(a)
        .add_systems(Startup, init_vector_field)
        .add_systems(Update, (update_field, update_arrows, move_charges).chain())
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Particle Sim".into(),
                        resolution: (1900., 1280.).into(),
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
}

fn init_vector_field(
    mut commands: Commands,
    mut vector_field: ResMut<VectorField>,
    asset_server: Res<AssetServer>,
) {
    let arrow_texture = asset_server.load("white_arrow.png");
    vector_field.init(&mut commands, arrow_texture, 50.0, 20.0);
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
fn move_charges(mut charges: ResMut<Charges>, vf: Res<VectorField>) {
    let time_scale = 10.0;
    let [width, height] = vf.get_shape();
    let bl = vf.coords[0][0];
    let tr = vf.coords[height - 1][width - 1];
    charges.update_velocities(time_scale);
    charges.move_charges(time_scale, [bl.x, tr.x, bl.y, tr.y]);
}
