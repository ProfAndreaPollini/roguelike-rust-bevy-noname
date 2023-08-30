use bevy::prelude::*;
use bevy_ecs_tilemap::{
    prelude::{get_tilemap_center_transform, TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage, TileTextureIndex},
};
use leafwing_input_manager::prelude::*;
use ron::de;

use crate::{
    algorithms::tile_pos_to_world_pos,
    bresenham_line,
    events::TurnEndEvent,
    intentions::{IntentionSourceRef, MoveIntention},
    resources::RLTimeSystem,
    GameState, IntentionKind, IntentionSourceId, IsVisited, MyGameCamera, Player, RLAction,
    TileKind, TileMapEntityLayer, TileMapLayer0, TileMapVisibilityLayer, VisibleTiles,
    VisitedTiles, Wall,
};
use bevy_prototype_debug_lines::*;
pub fn update_player(
    mut q: Query<
        (
            Entity,
            &mut Transform,
            &ActionState<RLAction>,
            &mut Player,
            &mut TilePos,
        ),
        With<Player>,
    >,
    mut tiles_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType), With<TileMapLayer0>>,
    //world: &World,
    walls_q: Query<(Entity, &TilePos), (With<Wall>, Without<Player>)>,
    mut commands: Commands,
) {
    // info!("update_player");
    if let Ok((map_size, grid_size, map_type)) = tiles_q.get_single_mut() {
        if let Ok((e, mut transform, action, mut player, mut tile_position)) = q.get_single_mut() {
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
                println!("Right");
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

            if desired_pos.x < 0 || desired_pos.x >= map_size.x as i32 {
                return;
            }

            if desired_pos.y < 0 || desired_pos.y >= map_size.y as i32 {
                return;
            }

            // player.tile_pos = TilePos::new(desired_pos.x as u32, desired_pos.y as u32);

            let pos = tile_pos_to_world_pos(
                &TilePos::new(desired_pos.x as u32, desired_pos.y as u32),
                map_size,
                grid_size,
                map_type,
            );
            // // println!("desired pos: {:?} | pos:
            //find the desidered tile

            // {:?}", desired_pos, pos);
            // let desired_tile = tiles_q.iter().find_map(|(size, grid_size, map_type)| {});
            //&TilePos::new(tile_position.x, tile_position.y));

            // if let Some(desired_tile) = desired_tile {
            //     let is_wall = walls_q.get(desired_tile).is_ok();
            //     if is_wall {
            //         println!("is_wall");
            //         return;
            //     }
            // }

            // if let Some(pos) = pos {
            //     transform.translation.x = pos.x as f32;
            //     transform.translation.y = pos.y as f32;
            // }
            // tile_position.x = desired_pos.x as u32;
            // tile_position.y = desired_pos.y as u32;
            if let Some(pos) = pos {
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
}

pub fn update_visibile_tiles(
    mut player_q: Query<(&Transform, &mut Player), With<Player>>,
    // layer0_q: Query<(&mut IsVisited, &TileStorage, &TilemapSize), With<TileMapLayer0>>,
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
    if let Ok((transform, mut player)) = player_q.get_single_mut() {
        // println!("visited_tiles: {:?}", player.visited_tiles.len());
        let player_position = Vec2::new(transform.translation.x, transform.translation.y);
        if let Ok(visibile_layer_data) = visibility_layer_q.get_single_mut() {
            // println!("ok {:?}", player_position);
            let (mut visibility_layer, size, grid_size, map_type) = visibile_layer_data;
            if let Some(player_cell) =
                TilePos::from_world_pos(&player_position, size, grid_size, map_type)
            {
                // println!("player_cell: {:?}", player_cell);
                // clean visible cells
                for cell in player.visible_tiles.iter() {
                    if let Some(cell) = visibility_layer.get(cell) {
                        commands.entity(cell).insert(TileTextureIndex(1));
                    }
                }
                player.visible_tiles.clear(); // clean visible cells

                let fov_size = 5;

                let mut fov_entitites: Vec<Entity> = vec![];

                // let x = -5;
                // let y = -5;
                let cell = IVec2::new(player_cell.x as i32, player_cell.y as i32);

                // let end = IVec2::new(cell.x as i32 + x, cell.y as i32 + y);

                // if cell.x < 0 || cell.y < 0 || cell.x >= size.x as i32 || cell.y >= size.y as i32 {
                //     return;
                // }
                // println!("start: {:?} | end: {:?}", cell, end);
                // let path = bresenham_line(cell, end, &size);

                // for cell in path {
                //     println!("cell: {:?}", cell);
                //     if let Some(cell_e) = visibility_layer.get(&cell) {
                //         commands.entity(cell_e).insert(TileTextureIndex(1));
                //     }
                // }
                for x in -fov_size..=fov_size {
                    for y in -fov_size..=fov_size {
                        if x == fov_size || x == -fov_size || y == fov_size || y == -fov_size {
                            // let cell =
                            //     IVec2::new(player_cell.x as i32 + x, player_cell.y as i32 + y);

                            // let end = IVec2::new(cell.x as i32 + x, cell.y as i32 + y);
                            // let path = bresenham_line(cell, end, &size);
                            // println!("path: {:?}", path);
                            // // if let Some(cell) = visibility_layer.get(&cell) {
                            // //     commands.entity(cell).insert(TileTextureIndex(1));
                            // // }
                            // for cell in path {
                            //     if let Some(cell) = visibility_layer.get(&cell) {
                            //         if !fov_entitites.contains(&cell) {
                            //             fov_entitites.push(cell);
                            //         }
                            //         commands.entity(cell).insert(TileTextureIndex(1));
                            //     }
                            // }
                            let end = IVec2::new(cell.x as i32 + x, cell.y as i32 + y);

                            if cell.x < 0
                                || cell.y < 0
                                || cell.x >= size.x as i32
                                || cell.y >= size.y as i32
                            {
                                return;
                            }
                            // println!("start: {:?} | end: {:?}", cell, end);
                            let path = bresenham_line(cell, end, &size);

                            for cell in path {
                                // println!("cell: {:?}", cell);
                                if let Some(cell_e) = visibility_layer.get(&cell) {
                                    if !fov_entitites.contains(&cell_e) {
                                        fov_entitites.push(cell_e);
                                        player.visible_tiles.push(cell);
                                    }
                                }
                            }
                        }
                    }
                }
                // println!("fov_entitites: {:?}", fov_entitites.len());
                for cell in fov_entitites {
                    commands.entity(cell).insert(TileTextureIndex(2));
                }
                // if player.visited_tiles.contains(&player_cell) {
                //     // println!("Player cell already visited: {:?}", player_cell);
                //     return;
                // } else {
                // }

                // if let Some(cell) = visibility_layer.get(&player_cell) {
                //     // println!("Player cell: {:?}", player_cell);
                //     player.visited_tiles.push(player_cell);
                //     commands.entity(cell).insert(TileTextureIndex(1));
                // }
            }
        }
    }
}

pub fn camera_follow(
    player_q: Query<(&TilePos, &Transform, &Player), With<Player>>,
    map_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType), With<TileMapLayer0>>,
    mut camera_q: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
    mut lines: ResMut<DebugLines>,
) {
    // info!("camera_follow");
    if let Ok((player_tile_pos, player_transform, _player)) = player_q.get_single() {
        if let Ok(mut camera_transform) = camera_q.get_single_mut() {
            let (map_size, grid_size, map_type) = map_q.get_single().unwrap();

            let centered_camera_transform =
                get_tilemap_center_transform(map_size, grid_size, map_type, 0.0);

            let player_world_pos =
                tile_pos_to_world_pos(player_tile_pos, map_size, grid_size, map_type).unwrap();

            let player_translation = player_transform.translation;

            let transform = centered_camera_transform.with_translation(Vec3::new(
                player_translation.x as f32,
                player_translation.y as f32,
                0.0,
            ));

            camera_transform.translation = transform.translation;
        }
    }
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
