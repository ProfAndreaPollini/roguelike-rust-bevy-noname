use bevy::prelude::Component;

mod input;
mod setup;
mod ui;
mod update;
// prelude

#[derive(Component)]
pub struct GameUiCamera;

pub mod prelude {
    pub use super::input::*;
    pub use super::setup::*;
    pub use super::ui::*;
    pub use super::update::*;
    pub use super::GameUiCamera;
}
