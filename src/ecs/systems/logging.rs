use bevy::{
    prelude::{info, Entity, Query, Res},
    time::Time,
};

use crate::ecs::components::position::Position;

pub fn log_positions(time: Res<Time>, query_positions: Query<(Entity, &Position)>) {
    info!("time delta: {:?}", time.delta());
    for (entity, position) in query_positions.iter() {
        info!("{:?} at {}", entity, position);
    }
}
