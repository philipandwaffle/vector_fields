use bevy::{
    app::{Plugin, Startup},
    ecs::system::Commands,
    hierarchy::BuildChildren,
    prelude::default,
    ui::{node_bundles::NodeBundle, JustifyContent, Style, Val},
};

use super::ui_elements::{ButtonBuilder, ButtonGroupBuilder};

pub enum ChargeEditorMode {
    Create,
    Move,
    Velocity,
    Charge,
}
#[derive()]
pub struct ChargeEditorState {
    mode: ChargeEditorMode,
}

pub struct ChargeEditorPlugin;
impl Plugin for ChargeEditorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let (button_width, button_height) = (24.5, 95.0);
    let button_builder = ButtonBuilder::new(
        Some("normal_text".to_string()),
        Some("hover_text".to_string()),
        Some("pressed_text".to_string()),
        button_width,
        button_height,
    );

    let (group_width, group_height) = (100.0, 20.0);
    let grouped_button_builder = ButtonGroupBuilder::new(
        vec![
            ["Place charges".into(), "Placing Charges".into()],
            ["Move charges".into(), "Moving Charges".into()],
            ["Alter velocities".into(), "Altering velocities".into()],
            ["Alter charge".into(), "Altering Charges".into()],
        ],
        group_width,
        group_height,
        button_builder,
    );

    let (mut button_group_ent, mut button_ents) = (None, None);
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        })
        .with_children(|p| {
            let (temp_button_group_ent, temp_button_ents) = grouped_button_builder.build(p);
            (button_group_ent, button_ents) = (Some(temp_button_group_ent), Some(temp_button_ents));
        });
    ButtonGroupBuilder::assign_button_group_component(&mut commands, button_group_ent, button_ents);
}
