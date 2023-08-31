use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::{
    prelude::{get_tilemap_center_transform, TilemapGridSize, TilemapId, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage, TileTextureIndex},
};
use leafwing_input_manager::prelude::*;

use crate::{
    algorithms::tile_pos_to_world_pos,
    bresenham_line,
    events::TurnEndEvent,
    intentions::{AttackIntention, IntentionSourceRef, MoveIntention},
    resources::RLTimeSystem,
    FovOccluder, GameState, IntentionKind, IntentionSourceId, IsVisited, Monster, MyGameCamera,
    NeedsFovUpdate, Player, RLAction, TileKind, TileMapEntityLayer, TileMapLayer0,
    TileMapVisibilityLayer, VisibleTiles, VisitedTiles, Wall,
};
use bevy_prototype_debug_lines::*;

type PlayerUpdateQueryData = (
    Entity,
    &'static mut Transform,
    &'static ActionState<RLAction>,
    &'static mut Player,
    &'static mut TilePos,
);
pub fn update_player(
    mut q: Query<PlayerUpdateQueryData, With<Player>>,
    mut tiles_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType), With<TileMapLayer0>>,
    //world: &World,
    walls_q: Query<(Entity, &TilePos), (With<Wall>, Without<Player>)>,
    monsters_q: Query<(Entity, &TilePos), (With<Monster>, Without<Wall>, Without<Player>)>,
    mut commands: Commands,
) {
    // info!("update_player");
    if let Ok((map_size, grid_size, map_type)) = tiles_q.get_single_mut() {
        if let Ok((e, transform, action, mut _player, tile_position)) = q.get_single_mut() {
            // println!("Player tile pos: {:?}", player.tile_pos);

            let mut dx = IVec2::default();
            if action.just_pressed(RLAction::Up) {
                // transform.translation.y += 10.;
                dx.y += 1;
            }
            if action.just_pressed(RLAction::Down) {
                // transform.translation.y -= 10.;
                dx.y -= 1;
            }
            if action.just_pressed(RLAction::Left) {
                // transform.translation.x -= 10.;
                dx.x -= 1;
            }

            if action.just_pressed(RLAction::Right) {
                // transform.translation.x += 10.;
                dx.x += 1;
            }

            if dx.length_squared() == 0 {
                return;
            }

            let desired_pos = IVec2::new(tile_position.x as i32, tile_position.y as i32) + dx;
            if desired_pos.length_squared() > 0 {
                info!("desired_pos: {:?}", desired_pos);
            }

            let pos = match tile_pos_to_world_pos(
                &TilePos::new(desired_pos.x as u32, desired_pos.y as u32),
                map_size,
                grid_size,
                map_type,
            ) {
                Some(pos) => pos,
                None => {
                    warn!("Player outside of map");
                    return;
                }
            };

            let monsters = monsters_q
                .iter()
                .filter(|(_, monster_tile_pos)| {
                    let monster_tile_pos = *monster_tile_pos;
                    monster_tile_pos.x == desired_pos.x as u32
                        && monster_tile_pos.y == desired_pos.y as u32
                })
                .collect::<Vec<_>>(); // mosters at desired position

            if !monsters.is_empty() {
                info!("Player wants to attack monster");
                for (monster_e, _) in monsters {
                    // commands.entity(monster_e).despawn_recursive();
                    commands.spawn(AttackIntention {
                        target: IntentionSourceRef(monster_e),
                        source: IntentionSourceRef(e),
                        target_pos: TilePos::new(desired_pos.x as u32, desired_pos.y as u32),
                    });
                }
                return;
            }

            let target_position = Vec3::new(
                // (pos.x as f32 + 0.5 * grid_size.x) as f32,
                // (pos.y as f32 + 0.5 * grid_size.y) as f32,
                pos.x as f32,
                pos.y as f32,
                transform.translation.z,
            );
            info!(
                "MoveIntention: {:?} wants to move to {:?}",
                e, target_position
            );
            commands.spawn((
                MoveIntention {
                    target: TilePos::new(desired_pos.x as u32, desired_pos.y as u32),
                    source: IntentionSourceRef(e),
                    target_position,
                },
                // IntentionSourceRef(e.0),
            ));
        }
    }
}

type Layer0OccludedTilesFilter = (
    With<TileMapLayer0>,
    // With<FovOccluder>,
    Without<TileMapVisibilityLayer>,
);

pub fn update_visibile_tiles(
    mut player_q: Query<(Entity, &Transform, &mut Player), (With<Player>, With<NeedsFovUpdate>)>,
    // layer0_q: Query<(&mut IsVisited, &TileStorage, &TilemapSize), With<TileMapLayer0>>,
    // mut fov_occluder_layer_q: Query<(&mut TileStorage,), Layer0OccludedTilesFilter>,
    fov_occluder_tiles_q: Query<(&TilePos), (With<TilemapId>, With<FovOccluder>)>,

    mut visibility_layer_q: Query<
        (
            &mut TileStorage,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
        ),
        With<TileMapVisibilityLayer>,
    >,

    mut commands: Commands,
) {
    // println!("update_visibile_tiles");
    let (player_entity, transform, mut player) = match player_q.get_single_mut() {
        Ok(player) => player,
        Err(_) => {
            return;
        }
    };

    let player_position = Vec2::new(transform.translation.x, transform.translation.y);
    let (visible_tiles_storage, size, grid_size, map_type) =
        match visibility_layer_q.get_single_mut() {
            Ok((visible_tiles_storage, size, grid_size, map_type)) => {
                (visible_tiles_storage, size, grid_size, map_type)
            }
            Err(_) => {
                warn!("No visibility layer found");
                return;
            }
        };

    let player_cell = match TilePos::from_world_pos(&player_position, size, grid_size, map_type) {
        Some(player_cell) => player_cell,
        None => {
            warn!("Player outside of map");
            return;
        }
    };

    let occluding_tiles = fov_occluder_tiles_q.iter().copied().collect::<Vec<_>>();

    // clean visible cells
    for cell in player.visible_tiles.iter() {
        if let Some(cell) = visible_tiles_storage.get(cell) {
            commands.entity(cell).insert(TileTextureIndex(1));
        }
    }
    player.visible_tiles.clear(); // clean visible cells

    let fov_size = 15;

    let cell = IVec2::new(player_cell.x as i32, player_cell.y as i32);

    let mut fov_tile_pos: HashSet<TilePos> = HashSet::new();
    let mut fov_entitites: HashSet<Entity> = HashSet::new();

    for x in -fov_size..=fov_size {
        for y in -fov_size..=fov_size {
            if x == fov_size || x == -fov_size || y == fov_size || y == -fov_size {
                let end = IVec2::new(cell.x as i32 + x, cell.y as i32 + y);

                if cell.x < 0 || cell.y < 0 || cell.x >= size.x as i32 || cell.y >= size.y as i32 {
                    return;
                }
                // println!("start: {:?} | end: {:?}", cell, end);
                let path = bresenham_line(cell, end, size);

                'outer: for cell in path {
                    if occluding_tiles.contains(&cell) {
                        // info!("cell {:?} is occluded", cell);
                        break 'outer;
                    } else {
                        let cell_visible_e = visible_tiles_storage.get(&cell).unwrap();

                        fov_entitites.insert(cell_visible_e);
                        fov_tile_pos.insert(cell);
                    }
                }
            }
        }
    }
    // println!("fov_entitites: {:?}", fov_entitites.len());
    for cell in fov_entitites {
        commands.entity(cell).insert(TileTextureIndex(2));
    }

    for cell in fov_tile_pos {
        player.visible_tiles.push(cell);
    }

    commands.entity(player_entity).remove::<NeedsFovUpdate>();
}

pub fn camera_follow(
    player_q: Query<(&Transform, &Player), With<Player>>,
    map_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType), With<TileMapLayer0>>,
    mut camera_q: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
    mut lines: ResMut<DebugLines>,
) {
    // info!("camera_follow");
    let (player_transform, _player) = match player_q.get_single() {
        Ok(player) => player,
        Err(_) => {
            warn!("No player found");
            return;
        }
    };

    let mut camera_transform = match camera_q.get_single_mut() {
        Ok(camera_transform) => camera_transform,
        Err(_) => {
            warn!("No camera found");
            return;
        }
    };
    let (map_size, grid_size, map_type) = map_q.get_single().unwrap();

    let centered_camera_transform =
        get_tilemap_center_transform(map_size, grid_size, map_type, 0.0);

    // let player_world_pos =
    //     tile_pos_to_world_pos(player_tile_pos, map_size, grid_size, map_type).unwrap();

    let player_translation = player_transform.translation;

    let transform = centered_camera_transform.with_translation(Vec3::new(
        player_translation.x,
        player_translation.y,
        0.0,
    ));

    camera_transform.translation = transform.translation;

    lines.line_colored(
        Vec3::new(-400.0, 0.0, 10.0),
        Vec3::new(400.0, 0.0, 10.0),
        0.9,
        Color::GREEN,
    );

    lines.line_colored(
        Vec3::new(0., -400.0, 10.0),
        Vec3::new(0., 400.0, 10.0),
        0.9,
        Color::RED,
    );
}

pub fn update_end_turn(
    mut time_system: ResMut<RLTimeSystem>,
    mut end_turn_er: EventReader<TurnEndEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    if !end_turn_er.is_empty() {
        println!("end_turn_er");
        time_system.increment();
        end_turn_er.clear();
        if game_state.get() == &GameState::PlayerTurn {
            *game_state = State::new(GameState::EnemyTurn);
        } else {
            *game_state = State::new(GameState::PlayerTurn);
        }
    }
}

pub fn update_enemies(mut game_state: ResMut<State<GameState>>) {}

pub fn update_entities(
    time_system: Res<RLTimeSystem>,
    entities_q: Query<(Entity, &IntentionKind)>,
) {
    let entities = time_system.get_entities_at_current_time();

    if let Some(entities) = entities {
        for entity in entities {
            println!("entity: {:?}", entity);
        }
    }
}
