use bevy::prelude::Event;

#[derive(Event, Debug, Clone, Copy)]
pub struct TurnEndEvent;

#[derive(Event, Debug, Clone, Copy, Default)]
pub struct IntentionEndEvent;
