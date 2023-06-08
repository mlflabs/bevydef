use bevy::{prelude::*, utils::HashMap};
// use bevy::{prelude::*};
use bevy_ecs_ldtk::prelude::*;
use std::collections::HashSet;

// use bevy_editor_pls::{EditorPlugin, default_windows::cameras::{camera_2d_panzoom::PanCamControls, EditorCamera}};
// use bevy_tilemap_editor_pls::TilemapEditorPlugin;

use crate::{player::components::{PlayerPosition, GridPosition, Player}, 
    settings::{GRID_LEVEL_PIXEL_WIDTH, GRID_LEVEL_PIXEL_HEIGHT, GRID_LEVEL_SIZE_START, GRID_LEVEL_SIZE_END}};

use super::components::LevelChangeEvent;



//const  map: HashMap<u128,u32> = HashMap::with_capacity(10);



pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    
    let mut iids: HashSet<String> = HashSet::new();
    for x in GRID_LEVEL_SIZE_START..GRID_LEVEL_SIZE_END + 1 {
        for y in GRID_LEVEL_SIZE_START..GRID_LEVEL_SIZE_END + 1 {
            let id =   ["Level_", &(x).to_string() ,"_", &(y).to_string()]
                .join("").replace("-", "0");    //("Level_" + (ev.x + x) as &str).to_owned();     
            
            println!("NEW ID: {:?}", id);
            iids.insert(id);

        }
    }

    println!("Map Setup done22332!!!!");
    commands.spawn(LdtkWorldBundle {
       ldtk_handle: asset_server.load("maps/level1.ldtk"),
       level_set: LevelSet { iids },
       transform: Transform::from_xyz(-256., -1024., 0.),
       ..Default::default()
    });
}


 

pub fn player_grid_change (
    mut ev_player_position: EventReader<PlayerPosition>,
    mut player_query: Query<&mut GridPosition, With<Player>>, //LevelChangeEvent
    mut ev_writer: EventWriter<LevelChangeEvent>
    //mut level_query: Query<&mut LevelSet>,
    //ldtk_assets: Res<Assets<LdtkAsset>>
){
    if let Ok(mut grid_position) = player_query.get_single_mut() {
        for ev in ev_player_position.iter(){
            let x: isize = (ev.value.x as isize / GRID_LEVEL_PIXEL_WIDTH) * GRID_LEVEL_PIXEL_WIDTH;
            let y: isize = (ev.value.y as isize / GRID_LEVEL_PIXEL_HEIGHT) * GRID_LEVEL_PIXEL_HEIGHT;

            let g = Vec2::new(x as f32, y as f32);

            if g != grid_position.pos {
                println!("Chenging grid position from {} to {}", grid_position.pos, g);
                grid_position.pos = g;
                //change level
                //let mut level_set = level_query.single_mut();
                println!("Player position changed to {}", ev.value);
                ev_writer.send(LevelChangeEvent { x: g.x as isize / GRID_LEVEL_PIXEL_WIDTH, y: g.y as isize / GRID_LEVEL_PIXEL_HEIGHT } )
            }
        } 
    }
}

 
pub fn level_change_system(
    mut ev_level_change: EventReader<LevelChangeEvent>,
    // mut level_selection: ResMut<LevelSelection>,
    mut level_sets: Query<&mut LevelSet>,
    //ldtk_assets: Res<Assets<LdtkAsset>>
)
{
    for ev in ev_level_change.iter(){
        
        
        let mut level_set = level_sets.single_mut();
        level_set.iids.clear();

        //let offsets: [isize] = []
        // let id =   ["Level_", &(ev.x).to_string() ,"_", &(ev.y).to_string()]
        //     .join("").replace("-", "0"); 
        // println!("Choosing level: {}", id);
        // *level_selection = LevelSelection::Iid(id); //::Iid(ldtk_level.level.iid.clone());

        for x in -1..2 {
            for y in -1..2 {
                println!("x, y: {}, {}", x, y);
                let id =   ["Level_", &(ev.x + x).to_string() ,"_", &(ev.y + y).to_string()]
                    .join("").replace("-", "0");    //("Level_" + (ev.x + x) as &str).to_owned();     
                
                println!("NEW ID: {:?}", id);
                level_set.iids.insert(id);

            }
        }
    }
}


// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut controls: Query<&mut PanCamControls, With<EditorCamera>>,
// ){

// } 