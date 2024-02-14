use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, Resource},
    },
    hierarchy::{BuildChildren, ChildBuilder},
    math::{vec2, vec3, Vec2},
    render::{color::Color, texture::Image},
    sprite::{Anchor, Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{controls::state::ControlState, setting::Settings};

#[derive(Component)]
pub struct Dragging;
#[derive(Component)]
pub struct ChargeIcon {
    pub id: usize,
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
impl IconBuilders {
    pub fn build_charge(&self, commands: &mut Commands, pos: Vec2, id: usize) -> (Entity, Entity) {
        let mut arrow_ent = Entity::PLACEHOLDER;
        let charge_ent = commands
            .spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::Hsla {
                            hue: 0.5,
                            saturation: 1.0,
                            lightness: 1.0,
                            alpha: 1.0,
                        },
                        custom_size: Some(vec2(self.charge.size, self.charge.size)),
                        anchor: self.charge.anchor,
                        ..default()
                    },
                    transform: Transform::from_translation(pos.extend(1.0)),
                    texture: self.charge.icon.clone(),
                    ..default()
                },
                ChargeIcon { id },
            ))
            .with_children(|p| {
                arrow_ent = self.build_arrow(p);
            })
            .id();

        (charge_ent, arrow_ent)
    }
    pub fn build_arrow(&self, commands: &mut ChildBuilder) -> Entity {
        commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::Hsla {
                        hue: 0.5,
                        saturation: 1.0,
                        lightness: 1.0,
                        alpha: 1.0,
                    },
                    custom_size: Some(vec2(self.arrow.size, self.arrow.size)),
                    anchor: self.arrow.anchor,
                    ..default()
                },
                transform: Transform {
                    translation: vec3(0.0, 0.0, 2.0),
                    scale: vec3(0.0, 1.0, 0.0),
                    ..default()
                },
                texture: self.arrow.icon.clone(),
                ..default()
            })
            .id()
    }
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

pub fn drag_icons(
    mut dragging: Query<&mut Transform, With<Dragging>>,
    control_state: Res<ControlState>,
) {
    for mut transfom in dragging.iter_mut() {
        let z = transfom.translation.z;
        transfom.translation = control_state.mouse_world_pos.extend(z);
    }
}
