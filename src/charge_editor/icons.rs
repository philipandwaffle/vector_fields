use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Res, Resource},
    },
    math::{vec2, Vec2},
    render::{color::Color, texture::Image},
    sprite::{Anchor, Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::setting::Settings;

#[derive(Component)]
pub struct Dragging;
#[derive(Component)]
pub struct ChargeIcon {
    id: usize,
}
#[derive(Component)]
pub struct ArrowIcon {
    id: usize,
}

#[derive(Resource)]
pub struct IconBuilders {
    pub charge: IconBuilder,
    pub arrow: IconBuilder,
}
impl IconBuilders {
    pub fn new(
        charge_icon: Handle<Image>,
        charge_size: f32,
        arrow_icon: Handle<Image>,
        arrow_size: f32,
    ) -> Self {
        Self {
            charge: IconBuilder {
                icon: charge_icon,
                size: charge_size,
                anchor: Anchor::Center,
            },
            arrow: IconBuilder {
                icon: arrow_icon,
                size: arrow_size,
                anchor: Anchor::CenterLeft,
            },
        }
    }
}

pub struct IconBuilder {
    icon: Handle<Image>,
    size: f32,
    anchor: Anchor,
}
impl IconBuilder {
    pub fn build_charge(&self, commands: &mut Commands, pos: Vec2, id: usize) -> Entity {
        commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::Hsla {
                            hue: 0.5,
                            saturation: 1.0,
                            lightness: 1.0,
                            alpha: 1.0,
                        },
                        custom_size: Some(vec2(self.size, self.size)),
                        anchor: self.anchor,
                        ..default()
                    },
                    texture: self.icon.clone(),
                    transform: Transform::from_translation(pos.extend(1.0)),
                    ..default()
                },
                ChargeIcon { id },
            ))
            .id()
    }
    pub fn build_arrow(commands: &mut Commands) {}
}

pub fn setup_builders(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    commands.insert_resource(IconBuilders::new(
        asset_server.load("charge.png"),
        settings.icons.charge_size,
        asset_server.load("white_arrow.png"),
        settings.icons.arrow_size,
    ))
}
