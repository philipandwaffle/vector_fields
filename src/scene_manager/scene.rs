use std::io::Error;

use bevy::ecs::system::Commands;

pub trait Scene {
    fn pre(commands: &mut Commands) -> Result<(), Box<Error>>;
    fn post(commands: &mut Commands) -> Result<(), Box<Error>>;
}
