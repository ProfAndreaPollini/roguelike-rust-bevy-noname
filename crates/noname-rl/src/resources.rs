#![allow(dead_code)]
use std::fmt::Display;

use bevy::{prelude::*, utils::HashMap};
use noise::NoiseFn;
#[derive(Default, Clone, PartialEq, Resource)]
pub struct GameContext {
    is_player_turn: bool,
}

#[derive(Default, Clone, PartialEq, Resource)]
pub struct RLRandomGenerator<T>
where
    T: NoiseFn<f64, 2>,
{
    pub noise: T,
}

impl<T: NoiseFn<f64, 2>> RLRandomGenerator<T> {
    pub fn new(noise: T) -> Self {
        Self { noise }
    }
}

#[derive(Default, Clone, PartialEq, Resource)]
pub struct RLTimeSystem {
    time: u32,
    schedule: HashMap<u32, Vec<Entity>>,
}

impl Display for RLTimeSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hours = self.time / 3600;
        let minutes = (self.time % 3600) / 60;
        let seconds = self.time % 60;
        write!(f, "{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl RLTimeSystem {
    pub fn new() -> Self {
        Self {
            time: 0,
            schedule: HashMap::default(),
        }
    }

    pub fn get_entities_at_time(&self, time: u32) -> Option<&Vec<Entity>> {
        self.schedule.get(&time)
    }

    pub fn get_entities_at_current_time(&self) -> Option<&Vec<Entity>> {
        self.get_entities_at_time(self.time)
    }
    pub fn schedule_entity(&mut self, entity: Entity, dt: u32) {
        let time = self.time + dt;
        if let Some(entities) = self.schedule.get_mut(&time) {
            entities.push(entity);
        } else {
            self.schedule.insert(time, vec![entity]);
        }
    }

    pub fn increment(&mut self) {
        self.time += 1;
    }

    pub fn get_time(&self) -> u32 {
        self.time
    }
}
