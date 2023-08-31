use bevy::{prelude::*, reflect::Reflect};
use bevy_ecs_tilemap::tiles::TilePos;
use leafwing_input_manager::Actionlike;

#[derive(Component, Default, Debug)]
pub struct WalkingAudioEffect {}

// stats components

#[derive(Component, Default, Debug)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Default, Debug)]
pub struct Attack {
    pub damage: i32,
}

#[derive(Bundle, Default, Debug)]
pub struct StatsBundle {
    pub health: Health,
    pub attack: Attack,
}

#[derive(Debug, Default, Clone, PartialEq, Component)]
pub struct Player {
    pub visited_tiles: Vec<TilePos>,
    pub visible_tiles: Vec<TilePos>,
    // pub tile_pos: TilePos,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum RLAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Default)]
pub struct VisitedTiles(pub Vec<TilePos>);

#[derive(Debug, Default, Clone, PartialEq, Component)]
pub struct VisibleTiles(pub Vec<TilePos>);

// #[derive(Component, Default)]
// pub struct EntityLayer(pub u32);

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub player: Player,
    pub tile_pos: TilePos,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    // pub input_manager: InputManagerBundle<RLAction>,
    pub visible_tiles: VisibleTiles,
    // pub visited_tiles: VisitedTiles,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Component)]
pub enum TileKind {
    #[default]
    Floor,
    Wall,
}

#[derive(Component, Default)]
pub struct FovOccluder;

#[derive(Component, Default)]
pub struct Wall;

#[derive(Bundle, Default)]
pub struct WallBundle {
    pub wall: Wall,
    pub fov_occluder: FovOccluder,
}

#[derive(Component, Default)]
pub struct NeedsFovUpdate;

// #[derive(Component, Default)]
// pub struct TileBundle {
//     pub tile_kind: TileKind,
// }

#[derive(Component, Default)]
pub struct TileMapVisibilityLayer;

#[derive(Component, Default)]
pub struct TileMapEntityLayer;

#[derive(Component, Default)]
pub struct TileMapLayer0;

#[derive(Component, Default)]
pub struct IsVisited;

// DEBUG

#[derive(Component)]
struct Axes;

#[derive(Component)]
pub enum IntentionKind {
    MoveTo { target: TilePos },
    Attack { target: Entity, target_pos: TilePos },
    Wait,
}

#[derive(Component)]
pub struct IntentionSourceId(pub Entity);

// Intensions

#[derive(Component, Default)]
pub struct TimeUIField {}

#[derive(Component, Default)]
pub struct TileInfoUI {}

#[derive(Component, Default)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Default)]
pub struct PlayerPositionUILabel {}

#[derive(Component, Default)]
pub struct TimeUIButton {}

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub enum ButtonStatus {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

#[derive(Component, Default, Clone, Copy, PartialEq, Debug, Reflect)]
pub struct HasTurn;

#[derive(Debug, Clone, PartialEq, Component)]
pub struct EntityRef(pub Entity);
