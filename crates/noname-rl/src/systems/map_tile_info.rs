use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use crate::{
    algorithms::tile_pos_to_world_pos, events::TileInfoEvent, MyGameCamera, TileMapLayer0,
};

pub fn my_cursor_system(
    buttons: Res<Input<MouseButton>>,
    // query to get camera transform
    camera_q: Query<(&Camera, &GlobalTransform), With<MyGameCamera>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut tiles_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType), With<TileMapLayer0>>,
    mut lines: ResMut<DebugLines>,
    mut tile_info_event: EventWriter<TileInfoEvent>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = camera_q.single();

    let (map_size, grid_size, map_type) = tiles_q.single_mut();

    if buttons.pressed(MouseButton::Right) {
        // Right Button is being held down

        // Games typically only have one window (the primary window)
        if let Some(position) = q_windows.single().cursor_position() {
            println!("Cursor is inside the primary window, at {:?}", position);
            let world_position = camera
                .viewport_to_world(camera_transform, position)
                .map(|ray| ray.origin.truncate())
                .unwrap();
            let p =
                TilePos::from_world_pos(&world_position, map_size, grid_size, map_type).unwrap();
            let world_position = tile_pos_to_world_pos(&p, map_size, grid_size, map_type).unwrap();

            let border_pos = Vec3::new(world_position.x as f32, world_position.y as f32, 1.)
                - Vec3::new(0.5 * grid_size.x, 0.5 * grid_size.y, 0.);

            lines.line_colored(
                Vec3::new(border_pos.x, border_pos.y, 1.),
                Vec3::new(border_pos.x + grid_size.x, border_pos.y, 1.),
                0.9,
                Color::GREEN,
            );
            lines.line_colored(
                Vec3::new(border_pos.x, border_pos.y, 1.),
                Vec3::new(border_pos.x, border_pos.y + grid_size.y, 1.),
                0.9,
                Color::GREEN,
            );
            lines.line_colored(
                Vec3::new(border_pos.x + grid_size.x, border_pos.y, 1.),
                Vec3::new(border_pos.x + grid_size.x, border_pos.y + grid_size.y, 1.),
                0.9,
                Color::GREEN,
            );
            lines.line_colored(
                Vec3::new(border_pos.x, border_pos.y + grid_size.y, 1.),
                Vec3::new(border_pos.x + grid_size.x, border_pos.y + grid_size.y, 1.),
                0.9,
                Color::GREEN,
            );

            println!(
                "World coords: {}/{} [tile: {:?}]",
                world_position.x, world_position.y, p
            );
            tile_info_event.send(TileInfoEvent { tile_pos: p });
        } else {
            println!("Cursor is not in the game window.");
        }
    }
}
