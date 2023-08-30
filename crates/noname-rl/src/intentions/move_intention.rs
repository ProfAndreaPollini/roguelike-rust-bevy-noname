use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TilePos;

use crate::{
    effects::prelude::PlayAudioEffect, IntentionKind, IntentionSourceId, MoveAction, MyAssets,
    WalkingAudioEffect,
};

use super::{IntentionResolver, IntentionSourceRef};

#[derive(Bundle)]
pub struct IntentionBundle {
    pub intention: IntentionKind,
    pub source: IntentionSourceId,
}

#[derive(Debug, Clone, PartialEq, Bundle)]
pub struct MoveIntentionBundle {
    pub intention: MoveIntention,
    // pub source: IntentionSourceRef,
}

#[derive(Debug, Clone, PartialEq, Component)]
pub struct MoveIntention {
    pub target: TilePos,
    pub source: IntentionSourceRef,
    pub target_position: bevy::prelude::Vec3,
}

impl IntentionResolver for MoveIntention {
    fn resolve_intention(
        &self,
        e: Entity,
        commands: &mut Commands,
        world: &World,
    ) -> Option<IntentionBundle> {
        // commands.entity(self.source.0).despawn_recursive();
        info!("MoveIntention: {:?}", self);
        if world.get_entity(e).is_some() {
            commands.add(MoveAction {
                target_tile: self.target,
                target_position: self.target_position,
                entity: self.source.0,
            });
            commands.add(PlayAudioEffect {
                effect: WalkingAudioEffect::default(),
            });

            commands.entity(e).despawn_recursive();
        } else {
            println!("MoveIntention: source entity is None");
        }
        None
    }
}
