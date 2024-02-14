use super::{
    icons::{ArrowIcon, ChargeIcon, IconBuilders},
    ui_elements::{ButtonBuilder, ButtonGroup, ButtonGroupBuilder},
};
use crate::{
    charge::{Charge, Charges},
    controls::state::ControlState,
    setting::Settings,
    utils,
};
use bevy::{
    ecs::{
        change_detection::DetectChanges,
        component::Component,
        entity::Entity,
        query::{Changed, With, Without},
        system::{Commands, Query, Res, ResMut, Resource},
    },
    hierarchy::{BuildChildren, Children},
    math::vec2,
    prelude::default,
    transform::components::Transform,
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
    cur_charge_index: Option<usize>,
    charge_icons: Vec<Entity>,
    arrow_icons: Vec<Entity>,
}
impl EditorState {
    pub fn new() -> Self {
        Self {
            mode: Mode::None,
            cur_charge_index: None,
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

pub fn update_icons(charges: Res<Charges>) {
    if !charges.is_changed() {
        return;
    }
}

pub fn if_create_charge(editor_state: Res<EditorState>) -> bool {
    return matches!(editor_state.mode, Mode::Create);
}
pub fn create_charge(
    mut commands: Commands,
    mut control_state: ResMut<ControlState>,
    mut charges: ResMut<Charges>,
    mut editor_state: ResMut<EditorState>,
    builders: Res<IconBuilders>,
    settings: Res<Settings>,
) {
    if !control_state.double_click {
        return;
    }
    control_state.double_click = false;

    let id = charges.charges.len();
    let world_pos = control_state.mouse_world_pos;
    let pos = world_pos / settings.simulation.scale;

    let (charge_ent, arrow_ent) =
        builders.build_charge(&mut commands, control_state.mouse_world_pos, id);

    editor_state.charge_icons.insert(id, charge_ent);
    editor_state.arrow_icons.insert(id, arrow_ent);

    charges
        .charges
        .insert(id, Charge::new(1.0, 1.0, pos, vec2(0.0, 0.0)));
}

pub fn if_move_charge(editor_state: Res<EditorState>) -> bool {
    return matches!(editor_state.mode, Mode::Move);
}
pub fn move_charge(
    mut charge_icons: Query<(&mut Transform, &ChargeIcon)>,
    mut charges: ResMut<Charges>,
    control_state: Res<ControlState>,
    settings: Res<Settings>,
) {
    if !control_state.left_mouse_down {
        return;
    }

    let mouse_world_pos = control_state.mouse_world_pos;
    for (mut transform, icon) in charge_icons.iter_mut() {
        let dir = mouse_world_pos - transform.translation.truncate();
        if dir.length_squared() > settings.icons.charge_size.powi(2) {
            continue;
        }

        let z = transform.translation.z;
        transform.translation = mouse_world_pos.extend(z);
        charges.charges[icon.id].p = mouse_world_pos / settings.simulation.scale;
        return;
    }
}

pub fn if_edit_velocity(editor_state: Res<EditorState>) -> bool {
    return matches!(editor_state.mode, Mode::Velocity);
}
pub fn edit_velocity(
    charge_icons: Query<(&Transform, &ChargeIcon)>,
    mut arrow_icons: Query<&mut Transform, (With<ArrowIcon>, Without<ChargeIcon>)>,
    mut editor_state: ResMut<EditorState>,
    mut charges: ResMut<Charges>,
    control_state: Res<ControlState>,
    settings: Res<Settings>,
) {
    if control_state.left_mouse_just_down {
        editor_state.cur_charge_index = None;
        for (transform, charge_icon) in charge_icons.iter() {
            let dir = control_state.mouse_world_pos - transform.translation.truncate();
            if dir.length_squared() > settings.icons.charge_size.powi(2) {
                continue;
            }

            editor_state.cur_charge_index = Some(charge_icon.id);
        }
    }

    if control_state.left_mouse_down {
        if let Some(i) = editor_state.cur_charge_index {
            let charge_pos = match charge_icons.get(editor_state.charge_icons[i]) {
                Ok((transform, _)) => transform.translation.truncate(),
                Err(err) => panic!("Charge not found when trying to edit velocity, {:?}", err),
            };

            let vel = (control_state.mouse_world_pos - charge_pos) / settings.simulation.scale;
            match arrow_icons.get_mut(editor_state.arrow_icons[i]) {
                Ok(mut transform) => {
                    transform.scale.x = vel.length();
                    transform.rotation = utils::dir_to_quat(vel);
                    charges.charges[i].v = vel;
                }
                Err(err) => panic!(
                    "Velocity arrow not found when trying to edit velocity, {:?}",
                    err
                ),
            }
        }
    }
}

pub fn if_edit_charge(editor_state: Res<EditorState>) -> bool {
    return matches!(editor_state.mode, Mode::Charge);
}
pub fn edit_charge() {}
