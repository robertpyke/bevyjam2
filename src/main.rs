mod ecs;

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
    time::FixedTimestep,
    window::PresentMode,
};
use ecs::systems;

const TIMESTEP_EXAMPLE: f64 = 1.0;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 900.,
            height: 900.,
            title: "BevyJam2".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(LogSettings {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=info".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::camera::setup_camera)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_EXAMPLE))
                .with_system(systems::example::hello_world),
        )
        .run();
}
