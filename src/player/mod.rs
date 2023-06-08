pub mod components;
mod systems;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;


use components::*;
use systems::*;



pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<PlayerActions>::default())
            .add_startup_system(spawn_player)   
            .add_event::<PlayerAction>()
            .add_event::<PlayerPosition>()
            .add_systems((player_walks, player_dash, cast_fireball, player_move, camera_move));
    }

}


