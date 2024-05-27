use bevy::prelude::*;
use world::WorldPlugin;
use player_plugin::PlayerPlugin;
use camera_plugin::ThirdPersonCameraPlugin;
use camera::CameraPlugin;
mod camera;
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
            CameraPlugin,
            ThirdPersonCameraPlugin, // Add it here
        ))
        .run();
}