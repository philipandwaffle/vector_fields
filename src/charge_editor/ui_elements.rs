use std::usize;

use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::{Component, TableStorage},
        entity::Entity,
        query::{Changed, With},
        system::{Commands, Query},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children},
    prelude::default,
    render::color::Color,
    text::{Text, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        BackgroundColor, BorderColor, Interaction, JustifyContent, Style, Val,
    },
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BACKGROUND_COLOR: Color = Color::hsla(180.0, 0.5, 0.5, 1.0);
const BORDER_COLOR: Color = Color::BLACK;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                update_grouped_button,
                update_button_cosmetics,
                update_grouped_button_cosmetics,
            ),
        );
    }
}
fn update_button_cosmetics(
    mut interaction_query: Query<
        (
            &ButtonMeta,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<ButtonMeta>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (meta, mut back_color, mut border_color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();

        match meta.cosmetic_state {
            CosmeticState::Pressed => {
                *back_color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
                if let Some(txt) = &meta.pressed_text {
                    text.sections[0].value = txt.clone();
                }
            }
            CosmeticState::Hovered => {
                *back_color = HOVERED_BUTTON.into();
                if let Some(txt) = &meta.hover_text {
                    text.sections[0].value = txt.clone();
                }
            }
            CosmeticState::None => {
                *back_color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
                if let Some(txt) = &meta.normal_text {
                    text.sections[0].value = txt.clone();
                }
            }
        }
    }
}

fn update_grouped_button_cosmetics(
    groups: Query<&ButtonGroup, Changed<ButtonGroup>>,
    mut buttons: Query<(&mut ButtonMeta, &GroupedButton)>,
) {
    for group in groups.iter() {
        for button_ent in group.button_ents.iter() {
            match buttons.get_mut(*button_ent) {
                Ok((mut meta, grouped_button)) => {
                    if group.cur_button.is_some() && grouped_button.id == group.cur_button.unwrap()
                    {
                        meta.cosmetic_state = CosmeticState::Pressed;
                    } else {
                        meta.cosmetic_state = CosmeticState::None;
                    }
                }
                Err(err) => panic!("Button in group doesn't exist: {:?}", err),
            }
        }
    }
}

fn update_grouped_button(
    mut groups: Query<&mut ButtonGroup>,
    interaction_query: Query<(&GroupedButton, &Interaction), (Changed<Interaction>, With<Button>)>,
) {
    for mut button_group in groups.iter_mut() {
        let button_ents = button_group.button_ents.clone();
        for button_ent in button_ents.iter() {
            let (grouped_button, interaction) = match interaction_query.get(*button_ent) {
                Ok(res) => res,
                Err(_) => continue,
            };

            if *interaction == Interaction::Pressed {
                (*button_group).set_cur_button(Some(grouped_button.id));
            }
        }
    }
}

#[derive(Component)]
pub struct ButtonGroup {
    pub cur_button: Option<usize>,
    button_ents: Vec<Entity>,
}
impl ButtonGroup {
    pub fn new(cur_button: Option<usize>, button_ents: Vec<Entity>) -> Self {
        Self {
            cur_button,
            button_ents,
        }
    }

    pub fn set_cur_button(&mut self, cur_button: Option<usize>) {
        self.cur_button = cur_button;
    }
}

#[derive(Clone, Component)]
pub struct GroupedButton {
    id: usize,
}

pub struct ButtonGroupBuilder {
    text: Vec<[String; 2]>,
    width: f32,
    height: f32,
    button_builder: ButtonBuilder,
}
impl ButtonGroupBuilder {
    pub fn new(
        text: Vec<[String; 2]>,
        width: f32,
        height: f32,
        button_builder: ButtonBuilder,
    ) -> Self {
        Self {
            text,
            width,
            height,
            button_builder,
        }
    }

    pub fn build(
        &self,
        cb: &mut ChildBuilder,
        tagging_component: impl Component<Storage = TableStorage>,
    ) -> (Entity, Vec<Entity>) {
        let mut button_ents = Vec::with_capacity(self.text.len());
        let mut button_group = cb.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(self.width),
                    height: Val::Percent(self.height),
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                background_color: BACKGROUND_COLOR.into(),
                ..default()
            },
            tagging_component,
        ));
        let button_group_ent = button_group.id();

        button_group.with_children(|p| {
            for i in 0..self.text.len() {
                let [normal, pressed] = self.text[i].clone();
                button_ents.push(self.button_builder.build_grouped(
                    p,
                    i,
                    ButtonMeta {
                        cosmetic_state: CosmeticState::None,
                        normal_text: Some(normal),
                        hover_text: None,
                        pressed_text: Some(pressed),
                    },
                ));
            }
        });

        (button_group_ent, button_ents)
    }

    // This is moronic but can't think of a better way to do this rn
    // TODO Figure out a better way to do this
    pub fn assign_button_group_component(
        commands: &mut Commands,
        button_group_ent: Option<Entity>,
        button_ents: Option<Vec<Entity>>,
    ) {
        if let Some(button_ents) = button_ents {
            if let Some(button_group_ent) = button_group_ent {
                commands
                    .entity(button_group_ent)
                    .insert(ButtonGroup::new(None, button_ents));
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum CosmeticState {
    Pressed,
    Hovered,
    None,
}

#[derive(Clone, Component)]
pub struct ButtonMeta {
    cosmetic_state: CosmeticState,
    normal_text: Option<String>,
    hover_text: Option<String>,
    pressed_text: Option<String>,
}

#[derive(Clone)]
pub struct ButtonBuilder {
    meta: ButtonMeta,
    width: f32,
    height: f32,
}
impl ButtonBuilder {
    pub fn new(
        normal_text: Option<String>,
        hover_text: Option<String>,
        pressed_text: Option<String>,
        width: f32,
        height: f32,
    ) -> Self {
        Self {
            meta: ButtonMeta {
                cosmetic_state: CosmeticState::None,
                normal_text,
                hover_text,
                pressed_text,
            },
            width,
            height,
        }
    }

    fn get_button_bundle(&self) -> ButtonBundle {
        ButtonBundle {
            button: bevy::ui::widget::Button,
            style: Style {
                width: Val::Percent(self.width),
                height: Val::Percent(self.height),
                bottom: Val::Percent((self.height - 100.0) / 2.0),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            background_color: NORMAL_BUTTON.into(),
            border_color: BORDER_COLOR.into(),
            ..default()
        }
    }
    fn get_text_bundle(&self) -> TextBundle {
        let text = match &self.meta.normal_text {
            Some(t) => t.clone(),
            None => "".to_string(),
        };

        TextBundle::from_section(
            text,
            TextStyle {
                font_size: 40.0,
                color: Color::rgb(0.9, 0.9, 0.9),
                ..default()
            },
        )
    }
    fn get_id(&self, id: usize) -> GroupedButton {
        GroupedButton { id }
    }

    pub fn build(&self, cb: &mut ChildBuilder) -> Entity {
        cb.spawn((self.get_button_bundle(), self.meta.clone()))
            .with_children(|p| {
                p.spawn(self.get_text_bundle());
            })
            .id()
    }

    pub fn build_grouped(&self, cb: &mut ChildBuilder, id: usize, meta: ButtonMeta) -> Entity {
        cb.spawn((self.get_button_bundle(), meta.clone(), self.get_id(id)))
            .with_children(|p| {
                p.spawn(self.get_text_bundle());
            })
            .id()
    }
}
