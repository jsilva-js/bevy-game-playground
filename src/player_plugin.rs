use bevy::prelude::*;

use crate::player::main_p::spawn_player;
use crate::player::mov::player_movement;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,spawn_player)
            .add_systems(Update, player_movement);
    }
}
