use bevy::{
    app::{Plugin, Update},
    ecs::{
        component::Component,
        query::{Changed, With},
        system::Query,
    },
    hierarchy::Children,
    prelude::default,
    render::color::Color,
    text::Text,
    ui::{
        node_bundles::ButtonBundle, widget::Button, BackgroundColor, BorderColor, Interaction,
        JustifyContent, Style, Val,
    },
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, button_system);
    }
}
fn button_system(
    mut interaction_query: Query<
        (&ButtonMeta, &Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (meta, interaction, mut color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                // border_color.0 = Color::RED;
                if let Some(txt) = meta.pressed_text.clone() {
                    text.sections[0].value = txt;
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                // border_color.0 = Color::WHITE;
                if let Some(txt) = meta.hover_text.clone() {
                    text.sections[0].value = txt;
                }
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                // border_color.0 = Color::BLACK;
                if let Some(txt) = meta.normal_text.clone() {
                    text.sections[0].value = txt;
                }
            }
        }
    }
}

#[derive(Clone, Component)]
pub struct ButtonMeta {
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
                normal_text,
                hover_text,
                pressed_text,
            },
            width,
            height,
        }
    }

    pub fn build(&self) -> (ButtonBundle, ButtonMeta) {
        return (
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
                ..default()
            },
            self.meta.clone(),
        );
    }
}
