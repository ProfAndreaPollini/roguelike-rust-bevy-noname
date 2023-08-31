#![allow(dead_code, unused_variables)]
use crate::{
    algorithms::tile_pos_to_world_pos,
    bresenham_line,
    resources::RLRandomGenerator,
    room::{self, Room},
    FovOccluder, GameState, MyAssets, NeedsFovUpdate, Player, PlayerBundle, RLAction, StatsBundle,
    TileKind, TileMapEntityLayer, TileMapLayer0, TileMapVisibilityLayer, VisitedTiles,
    WalkingAudioEffect, Wall, WallBundle,
};
use bevy::{prelude::*, render::camera::Viewport, transform::commands, ui::camera_config};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::egui::{plot::Line, Stroke};
use bevy_prototype_debug_lines::*;
use bevy_prototype_lyon::{
    prelude::{Fill, GeometryBuilder, Path, ShapeBundle},
    shapes,
};
use leafwing_input_manager::{prelude::InputMap, InputManagerBundle};
use noise::*;
use rand::{seq::SliceRandom, Rng};
use shape::*;

#[derive(Component, Default)]
pub struct MyGameCamera;

pub fn setup_player(assets: Res<MyAssets>, mut commands: Commands) {
    println!("setup_player");

    let mut input_map = InputMap::default();
    input_map.insert(KeyCode::Up, RLAction::Up);
    input_map.insert(GamepadButtonType::DPadUp, RLAction::Up);

    input_map.insert(KeyCode::Down, RLAction::Down);
    input_map.insert(GamepadButtonType::DPadDown, RLAction::Down);

    input_map.insert(KeyCode::Left, RLAction::Left);
    input_map.insert(GamepadButtonType::DPadLeft, RLAction::Left);

    input_map.insert(KeyCode::Right, RLAction::Right);
    input_map.insert(GamepadButtonType::DPadRight, RLAction::Right);

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: assets.sprites.clone(),
            sprite: TextureAtlasSprite {
                index: 220,
                custom_size: Some(Vec2::new(16., 16.)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 5.0),
            ..Default::default()
        },
        PlayerBundle {
            tile_pos: TilePos::new(15, 15),
            ..Default::default()
        },
        InputManagerBundle {
            input_map,
            ..Default::default()
        },
        NeedsFovUpdate,
        StatsBundle::default(),
    ));
}

pub fn setup_camera(
    mut commands: Commands,
    // mut q: Query<&mut OrthographicProjection, With<MyGameCamera>>,
) {
    println!("setup_camera");
    // do something using the asset handles from the resource
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // set the viewport to a 256x256 square in the top left corner
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(1100, 900),

                    ..default()
                }),

                ..default()
            },
            projection: OrthographicProjection {
                far: 1000.,
                scale: 0.5, // double the size of everything
                near: -1000.,

                ..Default::default()
            },
            ..default()
        },
        MyGameCamera,
        UiCameraConfig { show_ui: false },
    ));
    // commands.spawn((Camera2dBundle::default(), GameUiCamera));
}

/// Fills an entire tile storage with the given tile.
pub fn fill_positions(
    texture_index: TileTextureIndex,
    positions: Vec<TilePos>,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for tile_pos in positions {
            let tile_entity = parent
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id,
                    texture_index,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    });
}

use noise::Perlin;

pub fn map_noise(
    mut map_q: Query<
        (
            &mut TileStorage,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
        ),
        With<TileMapLayer0>,
    >,
    mut commands: Commands, // rng: Res<RLRandomGenerator<Fbm<Perlin>>>,
) {
    let (mut tile_storage, map_size, grid_size, map_type) = map_q.single_mut();

    // let mut noise_f : Fbm<Perlin> = rng.noise.clone();
    // noise_f = noise_f.set_seed(1);
    // noise_f = noise_f.set_octaves(4);
    // noise_f = noise_f.set_frequency(0.1);
    // noise_f = noise_f.set_lacunarity(2.0);
    // noise_f = noise_f.set_persistence(0.5);
    let perlin_noise = Perlin::default();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = tile_storage.get(&tile_pos).unwrap();

            let value = perlin_noise.get([
                (x as f32 / map_size.x as f32) as f64,
                (y as f32 / map_size.y as f32) as f64,
            ]) as f32;

            if value > 0. {
                commands.entity(tile_entity).insert((TileTextureIndex(205)));
            }
        }
    }
}

pub fn map_room_generator(
    mut map_q: Query<
        (
            &mut TileStorage,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapType,
        ),
        With<TileMapLayer0>,
    >,
    mut q: Query<(Entity, &mut Transform, &mut TilePos), With<Player>>,
    mut commands: Commands, // rng: Res<RLRandomGenerator<Fbm<Perlin>>>,
) {
    let (mut tile_storage, map_size, grid_size, map_type) = map_q.single_mut();
    let mut rooms = Vec::<Room>::new();

    let mut attempts = 0;
    let map_extent = map_size;
    while rooms.len() < 4 && attempts < 1000 {
        let candidate = Room::create_random_in_rect(
            IVec2::new(0, 0),
            IVec2::new(map_extent.x as i32, map_extent.y as i32),
            (10..25, 10..25),
        );

        if rooms.iter().all(|room| !candidate.intersects(room)) {
            rooms.push(candidate);
        }
        attempts += 1;
    }

    for room in rooms.iter() {
        for cell in room.border_cells() {
            let tile_pos = TilePos::new(cell.x as u32, cell.y as u32);
            if let Some(tile_entity) = tile_storage.checked_get(&tile_pos) {
                commands
                    .entity(tile_entity)
                    .insert((WallBundle::default(), TileTextureIndex(650)));
            }
        }

        for cell in room.interior_cells() {
            let tile_pos = TilePos::new(cell.x as u32, cell.y as u32);
            if let Some(tile_entity) = tile_storage.checked_get(&tile_pos) {
                commands
                    .entity(tile_entity)
                    .insert((TileKind::Floor, TileTextureIndex(4)))
                    .remove::<Wall>()
                    .remove::<FovOccluder>();
            }
        }
    }

    // connect rooms centers
    let mut corridor_tiles = Vec::<TilePos>::new();
    for i in 0..rooms.len() - 1 {
        let room_a = &rooms[i];
        let room_b = &rooms[i + 1];

        let start = room_a.center();
        let end = room_b.center();

        for i in -1..=1 {
            let positions = bresenham_line(
                IVec2::new(start.x + i as i32, start.y as i32),
                IVec2::new(end.x + i as i32, end.y as i32),
                map_size,
            );

            corridor_tiles.extend(positions);
        }
    }

    for tile in corridor_tiles.iter() {
        let tile_pos = TilePos::new(tile.x as u32, tile.y as u32);
        if let Some(tile_entity) = tile_storage.checked_get(&tile_pos) {
            commands
                .entity(tile_entity)
                .insert((TileKind::Floor, TileTextureIndex(54)))
                .remove::<Wall>()
                .remove::<FovOccluder>();
        }
    }

    // get a random cell in a random room
    let mut rng = rand::thread_rng();
    let room = rooms.choose(&mut rng).unwrap();
    let interior_cells = room.interior_cells();
    let cell = interior_cells.choose(&mut rng).unwrap();

    let pos = tile_pos_to_world_pos(
        &TilePos::new(cell.x as u32, cell.y as u32),
        map_size,
        grid_size,
        map_type,
    )
    .unwrap();

    let (e, mut player_transform, mut player_pos) = q.single_mut();

    player_pos.x = cell.x as u32;
    player_pos.y = cell.y as u32;

    player_transform.translation.x = pos.x as f32;
    player_transform.translation.y = pos.y as f32;

    dbg!(rooms);
}

pub fn map_setup(
    my_assets: Res<MyAssets>,
    // mut player_q: Query<(Entity, &mut Player), With<Player>>,
    mut commands: Commands,
    mut camera_q: Query<(&mut Transform, &Camera), (With<MyGameCamera>, Without<Player>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    // let (e, mut player) = player_q.get_single_mut().unwrap_or_else(|_| {
    //     panic!("There must be exactly one player entity with a Player component in the game world.")
    // });

    let (mut _camera_transform, camera) = camera_q.single_mut();
    let texture_handle: Handle<Image> = my_assets.player.clone();
    let visibility_image_handle = my_assets.visibility_image.clone();

    let map_size = TilemapSize { x: 320, y: 320 };

    // let viewport_size = camera
    //     .clone()
    //     .viewport
    //     .unwrap_or_default()
    //     .physical_size
    //     .clone();

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    // Layer 1
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(4),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );
    let mut rng = rand::thread_rng();

    for c in tile_storage.iter_mut() {
        if rng.gen::<f32>() > 0.7 {
            commands
                .entity(c.unwrap())
                .insert((WallBundle::default(), TileTextureIndex(35)));
        } else {
            // commands.entity(c.unwrap()).insert(TileKind::Floor);
        }
    }

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle.clone()),
            tile_size,
            // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        },
        TileMapLayer0,
    ));

    // // Layer 2
    // let mut tile_storage = TileStorage::empty(map_size);
    // let tilemap_entity = commands.spawn_empty().id();

    // // fill_tilemap(
    // //     TileTextureIndex(2),
    // //     map_size,
    // //     TilemapId(tilemap_entity),
    // //     &mut commands,
    // //     &mut tile_storage,
    // // );
    // let start = TilePos::new(0, 0);
    // let end = TilePos::new(7, 3);
    // // let tilemap_id = TilemapId(tilemap_entity);
    // let positions = bresenham_line(
    //     IVec2::new(start.x as i32, start.y as i32),
    //     IVec2::new(end.x as i32, end.y as i32),
    //     &map_size,
    // );

    // fill_positions(
    //     TileTextureIndex(34),
    //     positions,
    //     TilemapId(tilemap_entity),
    //     &mut commands,
    //     &mut tile_storage,
    // );

    // commands.entity(tilemap_entity).insert((TilemapBundle {
    //     grid_size,
    //     map_type,
    //     size: map_size,
    //     storage: tile_storage,
    //     texture: TilemapTexture::Single(texture_handle.clone()),
    //     tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
    //     // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0)
    //     //     * Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..Default::default()
    // },));

    // Visibiility Layer
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();

    fill_tilemap(
        TileTextureIndex(0),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(visibility_image_handle),
            tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
            // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 2.0)
            //     * Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        TileMapVisibilityLayer,
    ));

    // Visibiility Layer
    // let mut entity_tile_storage = TileStorage::empty(map_size);
    // let tilemap_entity = commands.spawn_empty().id();

    // fill_tilemap(
    //     TileTextureIndex(0),
    //     map_size,
    //     TilemapId(tilemap_entity),
    //     &mut commands,
    //     &mut tile_storage,
    // );

    // let tile_pos = TilePos::new(map_size.x / 2, map_size.y / 2);
    // entity_tile_storage.set(&tile_pos, e);

    // commands.entity(e).insert(TileBundle {
    //     position: tile_pos,
    //     tilemap_id: TilemapId(tilemap_entity),
    //     texture_index: TileTextureIndex(44),

    //     ..Default::default()
    // });

    // commands.entity(tilemap_entity).insert((
    //     TilemapBundle {
    //         grid_size,
    //         map_type,
    //         size: map_size,
    //         storage: entity_tile_storage,
    //         texture: TilemapTexture::Single(texture_handle),
    //         tile_size: TilemapTileSize { x: 16.0, y: 16.0 },
    //         transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 3.0)
    //             * Transform::from_xyz(0., 0., 0.0),
    //         // transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 3.0)
    //         //     * Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..Default::default()
    //     },
    //     TileMapEntityLayer,
    // ));

    *game_state = State::new(GameState::PlayerTurn);
}

pub fn setup_debug_layer(mut commands: Commands) {}

pub fn audio_effects_setup(assets: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: assets.load("sounds/steps-indoor-1.mp3"),
            // source: assets.load("sounds/walking1_gravel-6133.mp3"),
            ..Default::default()
        },
        WalkingAudioEffect::default(),
    ));
}
