use bevy::{ecs::system::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::vector_field::VectorField;

#[derive(Resource, Clone, Deserialize, Serialize)]
pub struct Charges {
    pub charges: Vec<Charge>,
}
impl Charges {
    pub fn new(charges: Vec<Charge>) -> Self {
        return Self { charges };
    }

    pub fn apply_to_field(&self, vector_field: &mut VectorField) {
        let [width, height] = vector_field.get_shape();
        for y in 0..height {
            for x in 0..width {
                let mut total = Vec2::ZERO;
                for c in &self.charges {
                    total += c.calc_e_force(1.0, vector_field.coords[y][x]);
                }
                vector_field.field[y][x] = total;
            }
        }
    }

    pub fn update_velocities(&mut self, dt: f32) {
        let num_charges = self.charges.len();
        let comp_charges = self.charges.clone();

        for cur_i in 0..num_charges {
            let cur_charge = &mut self.charges[cur_i];
            let mut vel = Vec2::ZERO;

            for comp_i in 0..num_charges {
                let compare_charge = &comp_charges[comp_i];

                if cur_i == comp_i {
                    continue;
                }

                vel -= cur_charge.calc_e_force(compare_charge.q, compare_charge.p);
            }
            cur_charge.v += vel * dt;
        }
    }

    pub fn move_charges(&mut self, dt: f32, bounds: [f32; 4]) {
        let [min_x, max_x, min_y, max_y] = bounds;
        for c in self.charges.iter_mut() {
            c.p += c.v * dt;

            if c.p.x < min_x {
                c.p.x = min_x + (min_x - c.p.x);
                c.v.x = -c.v.x
            }
            if c.p.x > max_x {
                c.p.x = max_x - (max_x - c.p.x);
                c.v.x = -c.v.x
            }
            if c.p.y < min_y {
                c.p.y = min_y + (min_y - c.p.y);
                c.v.y = -c.v.y
            }
            if c.p.y > max_y {
                c.p.y = max_y - (max_y - c.p.y);
                c.v.y = -c.v.y
            }
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Charge {
    q: f32,
    p: Vec2,
    v: Vec2,
}
impl Charge {
    pub fn new(q: f32, p: Vec2, v: Vec2) -> Self {
        return Self { q, p, v };
    }

    pub fn calc_e_force(&self, b_q: f32, b_p: Vec2) -> Vec2 {
        let dir = b_p - self.p;
        let dist_squared = dir.length_squared();
        let norm_dir = dir / f32::sqrt(dist_squared);
        let f = (self.q * b_q) / dist_squared;
        return norm_dir * f;
    }
}
