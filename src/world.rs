use bevy::prelude::*;
use self::{floor::spawn_floor, light::spawn_light};
mod light;
mod floor;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_light));
    }
}
