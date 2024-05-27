use bevy::prelude::*;
use world::WorldPlugin;
use player_plugin::PlayerPlugin;
use camera_plugin::ThirdPersonCameraPlugin;

mod camera_plugin;
mod world;
mod player_plugin;
mod player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin, // Add it here
        ))
        .run();
}