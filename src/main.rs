use bevy::prelude::*;
mod camera;
mod world;
mod player_plugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use player_plugin::PlayerPlugin;
mod player;
fn main() {
    App::new()
    .add_plugins((
        DefaultPlugins, 
        PlayerPlugin,
        CameraPlugin,
        WorldPlugin
    ))
    .run();
}
