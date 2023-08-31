use bevy::prelude::Component;

mod input;
mod map_tile_info;
mod monsters;
mod setup;
mod ui;
mod update;
// prelude

#[derive(Component)]
pub struct GameUiCamera;

pub mod prelude {
    pub use super::input::*;
    pub use super::map_tile_info::*;
    pub use super::monsters::*;
    pub use super::setup::*;
    pub use super::ui::*;
    pub use super::update::*;
    pub use super::GameUiCamera;
}
