use std::fmt::{Debug, Display, Formatter};

use bevy::{audio, ecs::system::Command, prelude::*};

use crate::WalkingAudioEffect;

#[derive(Component)]
pub struct PlayAudioEffect<T: Component + Debug + Default> {
    pub effect: T,
}

impl<T: Component + Debug + Default> Command for PlayAudioEffect<T> {
    fn apply(self, world: &mut World) {
        let mut audio_effect =
            world.query_filtered::<&Handle<AudioSource>, With<WalkingAudioEffect>>();
        info!("PlayAudioEffect: {:?}", self.effect);
        if let Ok(audio_effect) = audio_effect.get_single(world) {
            // audio_effect.
            world.spawn(AudioBundle {
                source: audio_effect.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

impl<T: Component + Debug + Default> Default for PlayAudioEffect<T> {
    fn default() -> Self {
        Self {
            effect: T::default(),
        }
    }
}

impl<T: Component + Debug + Default> Display for PlayAudioEffect<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "PlayAudioEffect: {:?}", self.effect)
    }
}

// pub fn audio_effects_system(assets: Res<MyAssets>,walking_effect: Query<&AudioSink, With<WalkingAudioEffect>>) {

// }
