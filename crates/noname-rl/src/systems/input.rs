use bevy::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{Player, RLAction};

pub fn setup_input_handler(
    mut input_map: Query<(Entity, &mut InputMap<RLAction>), With<Player>>,
    mut commands: Commands,
) {
    println!("input_handler");
    use RLAction::*;

    let (e, mut input_map) = input_map.get_single_mut().expect("player not found");

    input_map.insert(KeyCode::Up, Up);
    input_map.insert(GamepadButtonType::DPadUp, Up);

    input_map.insert(KeyCode::Down, Down);
    input_map.insert(GamepadButtonType::DPadDown, Down);

    input_map.insert(KeyCode::Left, Left);
    input_map.insert(GamepadButtonType::DPadLeft, Left);

    input_map.insert(KeyCode::Right, Right);
    input_map.insert(GamepadButtonType::DPadRight, Right);
}
