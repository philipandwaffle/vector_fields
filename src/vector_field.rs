use crate::utils;
use bevy::{
    asset::Handle,
    ecs::{
        entity::Entity,
        query::QueryEntityError,
        system::{Commands, Query, Resource},
    },
    math::{vec2, vec3, Vec2},
    prelude::default,
    render::{color::Color, texture::Image},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

#[derive(Resource)]
pub struct VectorField {
    size: [usize; 2],
    resolution: usize,
    pub field: Vec<Vec<Vec2>>,
    pub coords: Vec<Vec<Vec2>>,
    pub sprites: Vec<Vec<Option<Entity>>>,
}

impl VectorField {
    pub fn new(size: [usize; 2], resolution: usize) -> Self {
        let [width, height] = [size[0] * resolution, size[1] * resolution];
        let mut field = Vec::with_capacity(height);
        let mut sprites = Vec::with_capacity(height);
        let mut coords = Vec::with_capacity(height);

        for _ in 0..height {
            field.push(Vec::with_capacity(width));
            sprites.push(Vec::with_capacity(width));
            coords.push(Vec::with_capacity(width));
        }

        return Self {
            size,
            resolution,
            field,
            coords,
            sprites,
        };
    }

    pub fn init(
        &mut self,
        commands: &mut Commands,
        arrow_texture: Handle<Image>,
        spacing: f32,
        arrow_size: f32,
    ) {
        let [width, height] = self.get_shape();

        for y in 0..height {
            self.field.push(Vec::with_capacity(width));
            self.sprites.push(Vec::with_capacity(width));

            for x in 0..width {
                let coord = vec3(
                    (x as f32) - (width as f32 / 2.0),
                    (y as f32) - (height as f32 / 2.0),
                    0.0,
                ) * spacing
                    / self.resolution as f32;

                let sprite_ent = commands
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::Hsla {
                                hue: 0.5,
                                saturation: 1.0,
                                lightness: 1.0,
                                alpha: 1.0,
                            },
                            custom_size: Some(vec2(arrow_size, arrow_size)),
                            ..default()
                        },
                        texture: arrow_texture.clone(),
                        transform: Transform::from_translation(coord),
                        ..default()
                    })
                    .id();

                self.sprites[y].push(Some(sprite_ent));
                self.coords[y].push(coord.truncate());
                self.field[y].push(vec2(0.0, 1.0));
            }
        }
    }

    pub fn get_shape(&self) -> [usize; 2] {
        return [
            self.size[0] * self.resolution,
            self.size[1] * self.resolution,
        ];
    }

    fn normalise(&self) -> (Vec<Vec<Vec2>>, Vec<Vec<f32>>) {
        let [width, height] = self.get_shape();

        let mut mags = Vec::with_capacity(height);
        let mut norm_field = Vec::with_capacity(height);
        for row in self.field.iter() {
            let mut mag_row = Vec::with_capacity(width);
            let mut norm_row = Vec::with_capacity(width);

            for cell in row.iter() {
                let mag = cell.length();
                mag_row.push(cell.length());
                norm_row.push(*cell * (1.0 / mag));
            }

            mags.push(mag_row);
            norm_field.push(norm_row);
        }

        return (norm_field, mags);
    }

    pub fn update_sprites(
        &self,
        sprite_query: &mut Query<(&mut Sprite, &mut Transform)>,
    ) -> Result<(), QueryEntityError> {
        let [width, height] = self.get_shape();

        let (dir, mag) = self.normalise();
        for y in 0..height {
            for x in 0..width {
                if let Some(sprite_ent) = self.sprites[y][x] {
                    let (mut s, mut t) = sprite_query.get_mut(sprite_ent)?;

                    t.rotation = utils::dir_to_quat(dir[y][x]);
                    s.color = utils::mag_to_color(mag[y][x]);
                }
            }
        }

        Ok(())
    }
}
