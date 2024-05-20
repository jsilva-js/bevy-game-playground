use bevy::prelude::*;
mod player;
mod camera;
mod world;
use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(CameraPlugin)
    .add_plugin(WorldPlugin)
    .run();
}
