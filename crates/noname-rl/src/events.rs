use bevy::prelude::Event;
use bevy_ecs_tilemap::tiles::TilePos;

#[derive(Event, Debug, Clone, Copy)]
pub struct TurnEndEvent;

#[derive(Event, Debug, Clone, Copy, Default)]
pub struct IntentionEndEvent;

#[derive(Event, Debug, Clone, Copy)]
pub struct TileInfoEvent {
    pub tile_pos: TilePos,
}
