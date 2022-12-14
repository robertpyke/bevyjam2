mod ecs;

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
    time::FixedTimestep,
    window::PresentMode,
};

use ecs::systems;

const TIMESTEP_WORLD_SPAWNER: f64 = 1.0;
const TIMESTEP_NORM_TICK: f64 = 0.05;
const TIMESTEP_CLEANUP_TICK: f64 = 10.;
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
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=info".to_string(),
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(systems::camera::setup_camera)
        .add_startup_system(systems::spawner::spawn_initial_consumers)
        .add_startup_system(systems::ui::setup_ui)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_WORLD_SPAWNER))
                .with_system(systems::spawner::spawn_background_world_entities)
                .with_system(systems::spawner::spawn_producer_entities),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_NORM_TICK))
                .with_system(systems::movement::move_system)
                .with_system(systems::cleanup::cleanup_tick)
                .with_system(systems::consumption::consumption_system)
                .with_system(systems::specialization_transformer::specialization_transformer_system)
                .with_system(systems::renderer::transform_positions)
                .with_system(systems::logging::log_positions),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIMESTEP_CLEANUP_TICK))
                .with_system(systems::cleanup::cleanup_system),
        )
        .run();
}
