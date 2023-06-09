use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};



use crate::gamesystems::MainCamera;

use super::components::*;




pub const PLAYER_SPEED: f32 = 100.0;



pub fn spawn_player(
    mut commands: Commands,
) {

    

    let player_id = commands.spawn(PlayerBundle {
        player: Player,
        grid_position: GridPosition::default(),
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::default_input_map(),
            ..default()
        },
        sprite:SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 10.),
            sprite: Sprite {
                color: Color::rgb(0., 0.47, 1.),
                custom_size: Some(Vec2::new(10., 10.)),
                ..default()
            },
            ..default()
        },
        
        
        
    },
    ).id();


    let test = commands.spawn(SpriteBundle {
        sprite: Sprite { color: Color::RED, custom_size: Some(Vec2::new(5.0, 15.0)), ..default() },
        ..default()
    }).id();

    commands.entity(player_id).push_children(&[test]);

    // if let Ok(camera_entity) = camera_query.get_single() {
    //      commands.entity(player_id).push_children(&[camera_entity]);
    // }
}









pub fn player_walks(
    query: Query<&ActionState<PlayerActions>, With<Player>>,
    mut event_writer: EventWriter<PlayerAction>,
) {
    let action_state = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in PlayerActions::DIRECTIONS {
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += Vec2::from(direction);
                // println!("direction: {}", direction);
            }
        }
    }

    if direction_vector == Vec2::ZERO {
        return;
    }

    // Then reconvert at the end, normalizing the magnitude
    let net_direction: Result<Direction, NearlySingularConversion> = direction_vector.try_into();
    
    if let Ok(direction) = net_direction {
        // println!("direction: {}", direction);
        event_writer.send(PlayerAction { direction, action: PlayerActions::default() });
        // let dist = direction * time.delta_seconds() * PLAYER_SPEED;
        // transform.translation +=  Vec3::new(dist.x, dist.y, 0.);
    }
}



pub fn player_move(
    mut player_query: Query<&mut Transform, With<Player>>,
    mut ev_walk: EventReader<PlayerAction>,
    //mut camera_query: Query<&mut Transform, With<Camera>>,
    // mut camera: Query<(&mut PanOrbitCamera, &mut Transform), Without<Player>>,
    mut event_writer: EventWriter<PlayerPosition>,
    time: Res<Time>
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        
        //let mut player_translation= Vec3::ZERO;
        for ev in ev_walk.iter(){
            let dist = ev.direction.unit_vector() * time.delta_seconds() * PLAYER_SPEED;
            transform.translation += Vec3::from((dist, 0.0));
            //player_translation = transform.translation;

            event_writer.send(PlayerPosition { value: Vec2 { 
                x: transform.translation.x, y: transform.translation.y } })
        }

        // if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        //     //camera_transform.translation = player_translation;
        // }

    }
}

pub fn camera_move(
    player_query: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,    
){
    if let Ok (player_transform) = player_query.get_single() {
        if let Ok(mut camera) =  camera_query.get_single_mut() {
            camera.translation = player_transform.compute_transform().translation + Vec3::new(0.0, 0.0, 10.);
        }
    
    }
}


pub fn cast_fireball(query: Query<&ActionState<PlayerActions>, With<Player>>) {
    let action_state = query.single();

    if action_state.just_pressed(PlayerActions::Ability1) {
        println!("Fwoosh!");
    }
}

pub fn player_dash(query: Query<&ActionState<PlayerActions>, With<Player>>) {
    let action_state = query.single();

    if action_state.just_pressed(PlayerActions::Ability4) {
        let mut direction_vector = Vec2::ZERO;

        for input_direction in PlayerActions::DIRECTIONS {
            if action_state.pressed(input_direction) {
                if let Some(direction) = input_direction.direction() {
                    // Sum the directions as 2D vectors
                    direction_vector += Vec2::from(direction);
                }
            }
        }
        // Then reconvert at the end, normalizing the magnitude
        let net_direction: Result<Direction, NearlySingularConversion> =
            direction_vector.try_into();
        if let Ok(direction) = net_direction {
            println!("Dashing in {direction:?}");
        }
    }
}






