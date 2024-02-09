use std::error::Error;

use bevy::{ecs::system::Resource, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::json_parser::JSONParser;

#[derive(Resource, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub display: Display,
    pub simulation: Simulation,
}
impl Settings {
    pub fn set_display(&mut self, display: Display) {
        self.display = display;
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        JSONParser::save("assets/config/settings.cfg", self)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn Error>> {
        Ok(JSONParser::load("assets/config/settings.cfg")?)
    }
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Display {
    pub width: f32,
    pub height: f32,
}
impl Display {
    pub fn as_resolution(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Simulation {
    pub time_scale: f32,
    pub field: Field,
    pub vector: Vector,
}

#[derive(Copy, Clone, Deserialize, Serialize)]
pub struct Field {
    pub size: Vec2,
    pub resolution: f32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Vector {
    pub texture: String,
    pub size: f32,
    pub spacing: f32,
}
