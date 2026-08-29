use std::{ops::Add, time::Duration};
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_time::prelude::*;

pub struct TickPlugin;

impl Plugin for TickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_time));
        app.add_systems(
            Update,
            (update_time)
        );
    }
}

#[derive(Component)]
pub struct WorldTime {
    hours: i8,
    days: i32,
    last_tick: Duration,
    pub speed: i16,
    pub paused: bool,
}

pub fn setup_time(
    mut commands: Commands,
) {
    commands.spawn((
        WorldTime {
            hours: 0,
            days: 0,
            last_tick: Duration::ZERO,
            speed: 1,
            paused: false,
        },
    ));
}

pub fn update_time(
    time: Res<Time>,
    mut timer: Single<&mut WorldTime>,
) {
    let elapsed = time.elapsed();
    let diff = elapsed.abs_diff(timer.last_tick);
    if !timer.paused {
        if diff > Duration::from_millis((1000 / timer.speed / timer.speed) as u64) {
            timer.hours += 1;
            timer.last_tick = elapsed;
            hour_tick();
            if (timer.hours == 24) {
                timer.hours = 0;
                timer.days += 1;
                day_tick();
            }
        }
    }
}

pub fn hour_tick() {

}

pub fn day_tick() {
    
}
