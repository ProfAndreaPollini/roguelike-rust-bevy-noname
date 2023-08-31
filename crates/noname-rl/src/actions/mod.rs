use bevy::{a11y::accesskit::Vec2, ecs::system::Command, prelude::*, transform::commands};
use bevy_ecs_tilemap::{
    prelude::{TilemapGridSize, TilemapSize, TilemapType},
    tiles::{TilePos, TileStorage},
};
use bevy_tweening::{lens::TransformPositionLens, Animator, EaseFunction, Tween, TweenCompleted};

use crate::{
    algorithms::tile_pos_to_world_pos, events::IntentionEndEvent, NeedsFovUpdate,
    TileMapEntityLayer,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoveAction {
    pub target_tile: TilePos,
    pub target_position: bevy::prelude::Vec3,
    pub entity: Entity,
}

impl Command for MoveAction {
    fn apply(self, world: &mut World) {
        // info!("MoveAction: {:?}", self);

        // let mut pos: Option<Vec2> = None;
        // {
        //     let mut tiles = world.query_filtered::<(
        //         &mut TileStorage,
        //         &TilemapSize,
        //         &TilemapGridSize,
        //         &TilemapType,
        //     ), With<TileMapEntityLayer>>();

        //     let desired_pos =
        //         IVec2::new(self.target_position.x as i32, self.target_position.y as i32);

        //     let (tile_storage, map_size, grid_size, map_type) = tiles.single_mut(world);

        //     pos = tile_pos_to_world_pos(
        //         &TilePos::new(desired_pos.x as u32, desired_pos.y as u32),
        //         map_size,
        //         grid_size,
        //         map_type,
        //     );
        // }

        {
            let transform = world.get::<Transform>(self.entity).unwrap();

            let old_pos = transform.translation;

            // transform.translation.x = pos.x as f32;
            // transform.translation.y = pos.y as f32;
            let new_pos = Vec3::new(
                self.target_position.x,
                self.target_position.y,
                self.target_position.z,
            );

            // println!("MoveAction: {:?} => {:?}", old_pos, new_pos);
            let tween = Tween::new(
                EaseFunction::QuadraticInOut,
                std::time::Duration::from_millis(250),
                TransformPositionLens {
                    start: old_pos,
                    end: new_pos,
                },
            );
            // .with_user_data((self.target_tile, 66));
            // let on_completed =
            world.entity_mut(self.entity).insert(Animator::new(tween));

            // tween.set_completed(move |e, _tw| {
            //     println!("MoveAction: completed");
            //     world.send_event(IntentionEndEvent::default());
            //     // tw.despawn();
            // });
        }
        {
            let mut tile_pos = world.get_mut::<TilePos>(self.entity).unwrap();
            tile_pos.x = self.target_tile.x;
            tile_pos.y = self.target_tile.y;
        }
        {
            world.entity_mut(self.entity).insert(NeedsFovUpdate);
        }
    }
}

pub fn move_action_tween_end(mut reader: EventReader<TweenCompleted>, mut commands: Commands) {
    for ev in reader.iter() {
        println!(
            "Entity {:?} [{:?}] raised TweenCompleted!",
            ev.entity, ev.user_data
        );
        // match ev.user_data {
        //     (tile, 66) => {
        //         info!("MoveAction: completed : {:?}", tile);
        //     }
        //     _ => {}
        // }
    }
}
