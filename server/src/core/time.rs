use std::{ops::Add, time::Duration};
use bevy_ecs::prelude::*;
use bevy_app::prelude::*;
use bevy_replicon::shared::message::server_message::{SendTargets, ToClients};
use bevy_time::prelude::*;
use shared::{ClientMessage, ServerMessage, ProtocolPlugin};

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
    days: u8,
    weeks: u8,
    months: u8,
    years: u32,
    last_tick: Duration,
    pub speed: i16,
    pub paused: bool,
}

pub fn setup_time(
    mut commands: Commands,
) {
    commands.spawn((
        WorldTime {
            days: 1,
            weeks: 1,
            months: 1,
            years: 1920,
            last_tick: Duration::ZERO,
            speed: 1,
            paused: false,
        },
    ));
}

pub fn update_time(
    time: Res<Time>,
    mut timer: Single<&mut WorldTime>,
    mut server_messages: MessageWriter<ToClients<ServerMessage>>,
) {
    let elapsed = time.elapsed();
    let diff = elapsed.abs_diff(timer.last_tick);
    if !timer.paused {
        if diff > Duration::from_millis((1000 / timer.speed / timer.speed) as u64) {
            timer.days += 1;
            timer.last_tick = elapsed;
            
            day_tick();
            if (timer.days == 8) {
                timer.days = 1;
                timer.weeks += 1;
                week_tick();
            }
            if (timer.weeks == 5) {
                timer.weeks = 1;
                timer.months += 1;
            }
            if (timer.months == 13) {
                timer.months = 1;
                timer.years += 1;
            }
            server_messages.write(ToClients {
                targets: SendTargets::All,
                message: ServerMessage { msg: ("time ".to_string().add(&timer.days.to_string().add(&" ".to_string().add(&timer.weeks.to_string().add(&" ".to_string().add(&timer.months.to_string().add(&" ".to_string().add(&timer.years.to_string())))))))) },
            });
        }
    }
}

pub fn day_tick() {

}

pub fn week_tick() {
    
}
