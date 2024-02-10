use bevy::{
    app::{Plugin, Update},
    ecs::system::Commands,
    hierarchy::BuildChildren,
    prelude::default,
    render::color::Color,
    ui::{node_bundles::NodeBundle, JustifyContent, Style, Val},
};

use super::ui_elements::ButtonBuilder;

#[derive()]
pub struct ChargeEditor;
impl Plugin for ChargeEditor {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(20.0),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            background_color: Color::hsla(180.0, 0.5, 0.5, 1.0).into(),
            ..default()
        })
        .with_children(|p| {
            // let justify = JustifyContent::SpaceEvenly;
            let (width, height) = (24.5, 95.0);
            let builder = ButtonBuilder::new(
                Some("normal_text".to_string()),
                Some("hover_text".to_string()),
                Some("pressed_text".to_string()),
                width,
                height,
            );
            p.spawn(builder.build());
            p.spawn(builder.build());
            p.spawn(builder.build());
            p.spawn(builder.build());
        });
}
