use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Component)]
pub struct Cooldown<T> {
    pub timer: Timer,
    component: PhantomData<T>
}

impl<T> Cooldown<T> {
    pub fn new(seconds: f32) -> Self {
        Cooldown { 
            timer: Timer::from_seconds(seconds, TimerMode::Once),
            component: PhantomData
        }
    }
}