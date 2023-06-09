
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;


mod components;
mod systems;



use components::*;
use systems::*;




pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LdtkPlugin)
            //.insert_resource(LevelSelection::Index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior:: UseWorldTranslation {
                        load_level_neighbors: true,
                },
                ..Default::default()
                
            })
            
            .add_startup_system(setup)
            //.insert_resource(LevelSelection::Index(0))
            .register_ldtk_entity::<MyBundle>("Enemy1")
            .add_event::<LevelChangeEvent>()
            
            .add_system(level_change_system)
            .add_system(player_grid_change);
            
    }

}

 

#[derive(Default, Component)]
struct ComponentA;

#[derive(Default, Component)]
struct ComponentB;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: ComponentA,
    b: ComponentB,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
    #[worldly]
    worldly: Worldly,
}


