use bevy::{
    prelude::{info, Entity, Query, Res},
    time::Time,
};

use crate::ecs::components::{position::Position, velocity::Velocity};

pub fn log_positions(
    time: Res<Time>,
    query_positions: Query<(Entity, &Position, Option<&Velocity>)>,
) {
    info!("time delta: {:?}", time.delta());
    for (entity, position, optional_velocity) in query_positions.iter() {
        match optional_velocity {
            Some(velocity) => info!("{:?} at {} moving {}", entity, position, velocity),
            None => info!("{:?} at {}", entity, position),
        }
    }
}
