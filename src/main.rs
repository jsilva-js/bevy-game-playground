use bevy::prelude::*;
use world::WorldPlugin;
use player_plugin::PlayerPlugin;
use camera::CameraPlugin;
use bevy_third_person_camera::*;


mod camera;
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