#![allow(dead_code)]
use std::ops::Range;

use bevy::prelude::IVec2;
use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Room {
    pos: IVec2,
    size: IVec2,
}

impl Room {
    pub fn new(pos: IVec2, size: IVec2) -> Self {
        Self { pos, size }
    }

    pub fn border_cells(&self) -> Vec<IVec2> {
        let mut cells = Vec::<IVec2>::new();

        for x in self.pos.x..self.pos.x + self.size.x as i32 {
            cells.push(IVec2::new(x, self.pos.y));
            cells.push(IVec2::new(x, self.pos.y + self.size.y as i32 - 1));
        }

        for y in self.pos.y..self.pos.y + self.size.y as i32 {
            cells.push(IVec2::new(self.pos.x, y));
            cells.push(IVec2::new(self.pos.x + self.size.x as i32 - 1, y));
        }

        cells
    }

    pub fn interior_cells(&self) -> Vec<IVec2> {
        let mut cells = Vec::<IVec2>::new();

        for x in self.pos.x + 1..self.pos.x + self.size.x as i32 - 1 {
            for y in self.pos.y + 1..self.pos.y + self.size.y as i32 - 1 {
                cells.push(IVec2::new(x, y));
            }
        }

        cells
    }

    pub fn intersects(&self, other: &Room) -> bool {
        self.pos.x <= other.pos.x + other.size.x as i32
            && self.pos.x + self.size.x as i32 >= other.pos.x
            && self.pos.y <= other.pos.y + other.size.y as i32
            && self.pos.y + self.size.y as i32 >= other.pos.y
    }

    pub fn center(&self) -> IVec2 {
        IVec2::new(
            self.pos.x + self.size.x as i32 / 2,
            self.pos.y + self.size.y as i32 / 2,
        )
    }

    pub fn create_random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        let w = rng.gen_range(5..15);
        let h = rng.gen_range(5..15);

        Self::new(IVec2::new(x, y), IVec2::new(w, h))
    }

    pub fn create_random_in_rect(
        top_left: IVec2,
        size: IVec2,
        room_size_range: (Range<u32>, Range<u32>),
    ) -> Self {
        let mut rng = rand::thread_rng();

        let x = rng.gen_range(top_left.x..top_left.x + size.x);
        let y = rng.gen_range(top_left.y..top_left.y + size.y);

        let x_range_max = top_left.x + size.x as i32 - x;
        let x_range_min = room_size_range.0.start;

        let y_range_max = top_left.y + size.y as i32 - y;
        let y_range_min = room_size_range.1.start;

        let mut w: u32 = rng.gen_range(room_size_range.0).into();
        let mut h: u32 = rng.gen_range(room_size_range.1).into();
        // w = w.clamp(x_range_min, (x_range_max - 1) as u32) as u32;
        // h = h.clamp(y_range_min, (y_range_max - 1) as u32);

        Self::new(IVec2::new(x, y), IVec2::new(w as i32, h as i32))
    }

    pub fn cells(&self) -> Vec<IVec2> {
        let mut cells = Vec::<IVec2>::new();

        for x in self.pos.x..self.pos.x + self.size.x as i32 {
            for y in self.pos.y..self.pos.y + self.size.y as i32 {
                cells.push(IVec2::new(x, y));
            }
        }

        cells
    }
}
