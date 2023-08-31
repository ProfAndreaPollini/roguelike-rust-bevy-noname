use bevy::{a11y::accesskit::Vec2, prelude::IVec2};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::TilePos,
};

//TODO: Fix this
#[inline]
pub fn bresenham_line(
    start: IVec2,
    end: IVec2,
    size: &TilemapSize,
    // mut x1: IntVector2,
    // mut y1: IntVector2,
) -> Vec<TilePos> {
    use std::mem::swap;

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let mut x1 = end.x as i32;
    let mut y1 = end.y as i32;

    let steep = (x0 - x1).abs() < (y0 - y1).abs();
    // let reverse_output = x0 > x1;
    let start_x = x0;
    let start_y = y0;
    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2: i32 = dy.abs() * 2;
    let mut error2 = 0;
    let mut y = y0;

    let mut cells: Vec<TilePos> = vec![];

    let mut x = x0;
    while x <= x1 {
        // if x < 0 || y < 0 || x >= size.x as i32 || y >= size.y as i32 {
        //     continue;
        // }
        // print!("x: {}, y: {} || ", x, y);
        if steep {
            // image.set(y as usize, x as usize, color).ok();
            cells.push(TilePos::new(y as u32, x as u32));
        } else {
            // image.set(x as usize, y as usize, color).ok();
            cells.push(TilePos::new(x as u32, y as u32));
        }

        error2 += derror2;

        if error2 > dx {
            y += if y1 > y0 { 1 } else { -1 };
            error2 -= dx * 2;
        }
        x += 1;
    }
    // println!("cells: {:?}", cells);
    if cells[0].x != start_x as u32 && cells[0].y != start_y as u32 {
        // println!("cells[0] != x0 || cells[0] != y0");
        cells.reverse();
    }
    cells.retain(|cell| {
        if cell.x < 0 || cell.y < 0 || cell.x >= size.x || cell.y >= size.y {
            return false;
        }
        true
    });
    cells
}

pub fn tile_pos_to_world_pos(
    tile_pos: &TilePos,
    map_size: &TilemapSize,
    grid_size: &TilemapGridSize,
    map_type: &TilemapType,
) -> Option<Vec2> {
    match map_type {
        TilemapType::Square { .. } => {
            let x = ((tile_pos.x as f32 * grid_size.x) + 0.5);
            let y = ((tile_pos.y as f32 * grid_size.y) + 0.5);

            Some(Vec2::new(x as f64, y as f64))
        }
        _ => {
            println!("map_type: {:?}", map_type);
            None
        }
    }
}

pub mod prelude {
    pub use super::bresenham_line;
}
