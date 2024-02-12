use bevy::{
    ecs::{
        component::Component,
        query::{Changed, With},
        system::{Commands, Query, ResMut, Resource},
    },
    hierarchy::BuildChildren,
    prelude::default,
    ui::{node_bundles::NodeBundle, JustifyContent, Style, Val},
};

use super::ui_elements::{ButtonBuilder, ButtonGroup, ButtonGroupBuilder};

pub enum Mode {
    None,
    Create,
    Move,
    Velocity,
    Charge,
}
#[derive(Resource)]
pub struct EditorState {
    mode: Mode,
}
impl EditorState {
    pub fn new() -> Self {
        Self { mode: Mode::None }
    }
}

#[derive(Component)]
pub struct ButtonGroupTag;

pub fn spawn_ui(mut commands: Commands) {
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
            let (temp_button_group_ent, temp_button_ents) =
                grouped_button_builder.build(p, ButtonGroupTag);
            (button_group_ent, button_ents) = (Some(temp_button_group_ent), Some(temp_button_ents));
        });
    ButtonGroupBuilder::assign_button_group_component(&mut commands, button_group_ent, button_ents);
}

pub fn update_editor_mode(
    mut editor_state: ResMut<EditorState>,
    button_group: Query<&ButtonGroup, (Changed<ButtonGroup>, With<ButtonGroupTag>)>,
) {
    if let Ok(group) = button_group.get_single() {
        if let Some(cur_mode_id) = group.cur_button {
            let mut mode = Mode::None;

            if cur_mode_id == 0 {
                mode = Mode::Create
            } else if cur_mode_id == 1 {
                mode = Mode::Move
            } else if cur_mode_id == 2 {
                mode = Mode::Velocity
            } else if cur_mode_id == 3 {
                mode = Mode::Charge
            }

            editor_state.mode = mode;
        }
    }
}

fn place_charge() {}
fn move_charge() {}
fn edit_velocity() {}
fn edit_charge() {}
