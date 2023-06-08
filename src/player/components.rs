use bevy::prelude::*;

use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{orientation::Direction};


pub struct PlayerPosition {
    pub value: Vec2
}


pub struct PlayerAction {
    pub action: PlayerActions,
    pub direction: Direction,
}



// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum PlayerActions {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Abilities
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    Ultimate,
    None,
} 

impl Default for PlayerActions {
    fn default() -> Self { PlayerActions::None }
}



impl PlayerActions {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const DIRECTIONS: [Self; 4] = [
        PlayerActions::Up,
        PlayerActions::Down,
        PlayerActions::Left,
        PlayerActions::Right,
    ];

    pub fn direction(self) -> Option<Direction> {
        match self {
            PlayerActions::Up => Some(Direction::NORTH),
            PlayerActions::Down => Some(Direction::SOUTH),
            PlayerActions::Left => Some(Direction::WEST),
            PlayerActions::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}




#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct GridPosition {
    pub pos: Vec2,
}

impl Default for GridPosition {
    fn default() -> Self { GridPosition {pos: Vec2::ZERO} }
}


#[derive(Bundle)]
pub struct PlayerBundle {
    pub(crate) player: Player,
    pub(crate) grid_position: GridPosition,
    //pub(crate) transform: Transform,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub(crate) input_manager: InputManagerBundle<PlayerActions>,
    #[bundle]
    pub(crate) sprite: SpriteBundle
}


impl PlayerBundle {
    pub fn default_input_map() -> InputMap<PlayerActions> {
        // This allows us to replace `PlayerActions::Up` with `Up`,
        // significantly reducing boilerplate
        use PlayerActions::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);
        input_map.insert(KeyCode::W, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);
        input_map.insert(KeyCode::S, Down);


        input_map.insert(KeyCode::Left, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);
        input_map.insert(KeyCode::A, Left);


        input_map.insert(KeyCode::Right, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);
        input_map.insert(KeyCode::D, Right);


        // Abilities
        input_map.insert(KeyCode::Q, Ability1);
        input_map.insert(GamepadButtonType::West, Ability1);
        input_map.insert(MouseButton::Left, Ability1);

        //input_map.insert(KeyCode::W, Ability2);
        input_map.insert(GamepadButtonType::North, Ability2);
        input_map.insert(MouseButton::Right, Ability2);

        input_map.insert(KeyCode::E, Ability3);
        input_map.insert(GamepadButtonType::East, Ability3);

        input_map.insert(KeyCode::Space, Ability4);
        input_map.insert(GamepadButtonType::South, Ability4);

        input_map.insert(KeyCode::R, Ultimate);
        input_map.insert(GamepadButtonType::LeftTrigger2, Ultimate);

        input_map
    }
}

