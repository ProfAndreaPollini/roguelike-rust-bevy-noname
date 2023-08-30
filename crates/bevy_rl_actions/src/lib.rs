#![allow(dead_code)]
use bevy::{
    prelude::{Component, Entity, Plugin, ReflectComponent},
    reflect::Reflect,
};

/// A bevy tilemap plugin. This must be included in order for everything to be rendered.
/// But is not necessary if you are running without a renderer.
pub struct RlEntityActionsPlugin;

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct IntentId(pub Entity);

impl Default for IntentId {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct TargetId(pub Entity);

impl Default for TargetId {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

#[derive(Component, Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
pub struct ActionId(pub Entity);

impl Default for ActionId {
    fn default() -> Self {
        Self(Entity::from_raw(0))
    }
}

impl Plugin for RlEntityActionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // app.add_systems(First, update_changed_tile_positions);

        app.register_type::<IntentId>();
        app.register_type::<TargetId>();
    }
}
