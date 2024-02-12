use super::{
    icons::{IconBuilder, IconBuilders},
    ui_elements::{ButtonBuilder, ButtonGroup, ButtonGroupBuilder},
};
use crate::{
    charge::{self, Charge, Charges},
    controls::state::ControlState,
    setting::Settings,
};
use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::BuildChildren,
    math::{vec2, Vec2},
    prelude::default,
    ui::{node_bundles::NodeBundle, JustifyContent, Style, Val},
};

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
    charge_icons: Vec<Entity>,
    arrow_icons: Vec<Entity>,
}
impl EditorState {
    pub fn new() -> Self {
        Self {
            mode: Mode::None,
            charge_icons: vec![],
            arrow_icons: vec![],
        }
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

pub fn handle_input(
    mut commands: Commands,
    mut control_state: ResMut<ControlState>,
    mut charges: ResMut<Charges>,
    mut editor_state: ResMut<EditorState>,
    builders: Res<IconBuilders>,
    settings: Res<Settings>,
) {
    match editor_state.mode {
        Mode::None => return,
        Mode::Create => {
            if control_state.double_click {
                control_state.double_click = false;

                let id = charges.charges.len();
                let world_pos = control_state.mouse_world_pos;
                let pos = world_pos / settings.simulation.scale;

                let charge_ent =
                    builders
                        .charge
                        .build_charge(&mut commands, control_state.mouse_world_pos, id);
                editor_state.charge_icons.insert(id, charge_ent);

                charges
                    .charges
                    .insert(id, Charge::new(1.0, 1.0, pos, vec2(0.0, 0.0)));
            }
        }
        Mode::Move => todo!(),
        Mode::Velocity => todo!(),
        Mode::Charge => todo!(),
    }
}

fn move_charge() {}
fn edit_velocity() {}
fn edit_charge() {}
