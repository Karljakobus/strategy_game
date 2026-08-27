use std::{ops::Add, time::Duration};
use crate::render::text_display::{self, create_text_ur, delete_display};
use crate::render::text_display::*;

use bevy::{prelude::*,};

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

    create_text_ur(&mut commands, ("0:00 - day: ".to_string() + &0.to_string()).to_string(), "time".to_string(), 12, 12);
    create_text_ur(&mut commands, ("speed: ".to_string() + &1.to_string()).to_string(), "speed".to_string(), 12, 42);
}

pub fn update_time(
    time: Res<Time>,
    mut timer: Single<&mut WorldTime>,
    mut displays: Query<(&TextSystem, &mut Text)>,
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
            modify_display(timer.hours.to_string().add(&":00 - day: ".to_string().add(&timer.days.to_string())), &mut displays, "time".to_string());
        }
        modify_display("speed :".to_string().add(&timer.speed.to_string()), &mut displays, "speed".to_string());
    } else {
        modify_display("PAUSED".to_string(), &mut displays, "speed".to_string());
    }
}

pub fn hour_tick() {

}

pub fn day_tick() {
    
}