use bevy::prelude::*;
mod player;
mod camera;
mod light;
mod floor;
use player::PlayerPlugin;
use camera::CameraPlugin;
use light::LightPlugin;
use floor::FloorPlugin;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(CameraPlugin)
    .add_plugin(LightPlugin)
    .add_plugin(FloorPlugin)
    .run();
}
