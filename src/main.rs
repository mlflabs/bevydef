use bevy::prelude::*;



mod gamesystems;
mod player;
mod map;
mod settings;

use gamesystems::GameSystemsPlugin;
use player::PlayerPlugin;
use map::MapPlugin;



fn main() {
    App::new()
        .add_plugin(GameSystemsPlugin)
        
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .run();
}

