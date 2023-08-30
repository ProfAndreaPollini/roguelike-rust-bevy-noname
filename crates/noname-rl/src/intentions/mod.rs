use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{components::EntityRef, MoveAction};

#[derive(Debug, Clone, PartialEq, Component)]
pub struct IntentionSourceRef(pub Entity);

mod move_intention;
pub use move_intention::*;

pub mod prelude {
    use super::*;
    use move_intention::*;
}

/// IntentionResolver
///
/// IntentionResolver is a trait that is used to resolve intentions.
pub trait IntentionResolver {
    fn resolve_intention(
        &self,
        e: Entity,
        commands: &mut Commands,
        world: &World,
    ) -> Option<IntentionBundle>;
}

#[derive(Debug, Clone, PartialEq, Component)]
pub struct AttackIntention {
    pub target: EntityRef,
    pub target_pos: TilePos,
    pub source: EntityRef,
}

pub fn process_move_intention(
    entities_q: Query<(Entity, &MoveIntention)>,
    walls_q: Query<&TilePos, With<crate::components::Wall>>,
    mut commands: Commands,
    world: &World,
) {
    for (entity, intention) in entities_q.iter() {
        //check if there is a wall
        let mut is_blocked = false;
        for wall_pos in walls_q.iter() {
            if wall_pos == &intention.target {
                is_blocked = true;
                //continue 'outer;
                break;
            }
        }

        // commands.despawn(entity);
        if is_blocked {
            info!("tile {:?} is not accessible", intention.target);
        } else {
            println!(
                "process_move_intention: {:?} for entity {:?}",
                intention, entity
            );
            intention.resolve_intention(entity, &mut commands, world);
        }
        commands.entity(entity).despawn_recursive();
    }
}
