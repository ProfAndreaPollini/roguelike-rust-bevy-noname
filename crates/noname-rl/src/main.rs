use std::os::windows::process;

use bevy::{
    input::common_conditions::{input_pressed, input_toggle_active},
    prelude::*,
    window::WindowResolution,
};
use bevy_asset_loader::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use bevy_prototype_lyon::prelude::*;
use bevy_tweening::{lens::*, *};
use events::{IntentionEndEvent, TileInfoEvent, TurnEndEvent};
use intentions::{process_attack_intention, process_move_intention};
use leafwing_input_manager::prelude::*;
use noise::*;

mod actions;
mod algorithms;
mod components;
mod effects;
mod events;
mod intentions;
mod query;
mod resources;
mod room;
mod systems;

pub use actions::*;
pub use algorithms::prelude::*;
pub use query::*;

pub use components::*;
use resources::{RLRandomGenerator, RLTimeSystem};
pub use systems::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    AssetLoading,
    AssetsLoaded,
    PlayerTurn,
    EnemyTurn,
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    // #[asset(path = "urizen_onebit_tileset__v1d0.png")]
    #[asset(path = "kenney_1-bit-pack/Tilesheet/colored_packed.png")]
    player: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 49, rows = 22))]
    #[asset(path = "kenney_1-bit-pack/Tilesheet/colored_packed.png")]
    pub sprites: Handle<TextureAtlas>,

    #[asset(path = "out.png")]
    visibility_image: Handle<Image>,

    // #[asset(path = "sounds/steps-indoor-1-6723.mp3")]
    // walking: Handle<AudioSource>,

    // #[asset(path = "walking.ogg")]
    // walking: Handle<AudioSource>,
    #[asset(path = "fonts/dealerplate_california.otf")]
    ui_font: Handle<Font>,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Noname RL"),
                        resolution: WindowResolution::new(1500., 1000.),
                        resize_constraints: WindowResizeConstraints {
                            min_width: 1500.,
                            min_height: 1000.,
                            ..Default::default()
                        },
                        resizable: false,
                        ..Default::default()
                    }),

                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .insert_resource(Msaa::Sample4)
        .insert_resource(RLTimeSystem::new())
        .insert_resource(RLRandomGenerator::new(Fbm::<Perlin>::new(0)))
        // events:
        .add_event::<TurnEndEvent>()
        .add_event::<IntentionEndEvent>()
        .add_event::<TileInfoEvent>()
        .add_plugins(DebugLinesPlugin::default())
        .add_plugins(ShapePlugin)
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::AssetsLoaded),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
        .add_systems(
            OnEnter(GameState::AssetsLoaded),
            (
                // use_my_assets,
                game_ui_setup,
                audio_effects_setup,
                (
                    // apply_deferred,
                    setup_camera,
                    apply_deferred,
                    map_setup,
                    apply_deferred,
                    setup_player,
                    apply_deferred,
                    map_noise,
                    map_room_generator,
                )
                    .chain(),
                apply_deferred,
                spawn_monster,
                setup_input_handler.after(setup_player),
            )
                .chain(),
        )
        .add_plugins(InputManagerPlugin::<RLAction>::default())
        .add_plugins(
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        // .add_systems(Startup, (setup_camera, setup_input_handler.after(setup_player)))
        // .add_systems(PreUpdate,)
        .add_systems(
            Update,
            (
                update_player.run_if(state_exists_and_equals(GameState::PlayerTurn)),
                update_enemies.run_if(state_exists_and_equals(GameState::EnemyTurn)),
                camera_follow,
                apply_deferred,
                update_visibile_tiles,
                apply_deferred,
                process_move_intention,
                process_attack_intention,
                my_cursor_system.run_if(input_pressed(MouseButton::Right)),
                apply_deferred,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            (
                update_end_turn,
                game_ui_update,
                game_ui_player_position_update,
                game_ui_interaction,
                move_action_tween_end,
                ui_update_on_query_tile_event,
            ),
        )
        .add_plugins(TweeningPlugin)
        // .add_system s(PostUpdate, draw_game_ui)
        .run();
}
