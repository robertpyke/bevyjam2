use bevy::{
    prelude::{debug, Entity, Query, Res},
    time::Time,
};

use crate::ecs::components::{position::Position, title::Title, velocity::Velocity};

pub fn log_positions(
    time: Res<Time>,
    query_positions: Query<(Entity, &Position, Option<&Title>, Option<&Velocity>)>,
) {
    debug!("time delta: {:?}", time.delta());
    for (entity, position, optional_title, optional_velocity) in query_positions.iter() {
        let log_string = match (optional_title, optional_velocity) {
            (None, None) => format!("({:?}) at {}", entity, position),
            (None, Some(velocity)) => format!("({:?}) at {} moving {}", entity, position, velocity),
            (Some(title), None) => format!("{} ({:?}) at {}", title, entity, position),
            (Some(title), Some(velocity)) => format!(
                "{} ({:?}) at {} moving {}",
                title, entity, position, velocity
            ),
        };
        debug!("{}", log_string);
    }
}
