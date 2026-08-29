use std::time::Duration;
use server::core::map_gen;
use bevy_app::{ScheduleRunnerPlugin, prelude::*};
use bevy_state::app::StatesPlugin;
use bevy_internal::prelude::*;
use server::core::time::TickPlugin;
use server::core::net::NetworkPlugin;
fn main() {
    let map = map_gen::map_gen(20, 20, 42);
    println!("Generated map with {} provinces", map.provinces.len());

    App::new()
        .insert_resource(Time::from_hz(30.0))
        .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
            Duration::from_secs_f64(1. / 60.),
        )))
        .add_plugins(StatesPlugin)
        .add_plugins(TickPlugin)
        .add_plugins(NetworkPlugin)
        .run();
}

