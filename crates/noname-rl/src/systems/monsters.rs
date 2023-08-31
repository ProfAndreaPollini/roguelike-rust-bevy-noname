use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::Rng;

use crate::{algorithms::tile_pos_to_world_pos, MyAssets, StatsBundle, TileMapLayer0, Wall};

#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle, Default)]
pub struct MonsterBundle {
    pub monster: Monster,
    // pub visible_tiles: VisibleTiles,
    // pub visited_tiles: VisitedTiles,
    // pub tile_pos: TilePos,
    // pub transform: Transform,
    // pub global_transform: GlobalTransform,
    // pub animator: Animator<RLAction>,
    // pub needs_fov_update: NeedsFovUpdate,
    // pub walking_audio_effect: WalkingAudioEffect,
}

pub fn spawn_monster(
    mut commands: Commands,
    assets: Res<MyAssets>,
    map_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType), With<TileMapLayer0>>,
    floor_tiles_q: Query<(&TilePos), (With<TilemapId>, Without<Wall>)>,
) {
    let floor_tiles: Vec<TilePos> = floor_tiles_q.iter().copied().collect();

    let n_monsters = 100;

    for _ in 0..n_monsters {
        let (map_size, grid_size, map_type) = map_q.single();

        let mut rng = rand::thread_rng();

        let tile_pos = floor_tiles[rng.gen_range(0..floor_tiles.len())];

        let pos = tile_pos_to_world_pos(&tile_pos, map_size, grid_size, map_type).unwrap();

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: assets.sprites.clone(),
                sprite: TextureAtlasSprite {
                    index: 25,
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(pos.x as f32, pos.y as f32, 6.0),
                ..Default::default()
            },
            MonsterBundle::default(),
            TilePos::new(tile_pos.x, tile_pos.y),
            Name::new("Monster"),
            StatsBundle::default(),
        ));
    }
}
