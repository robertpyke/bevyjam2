use bevy::prelude::{debug, Commands, Entity, Query};

use crate::ecs::components::{position::Position, title::Title};

const CLEANUP_BOUNDS: f32 = 4000.;

pub fn cleanup_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, Option<&Title>)>,
) {
    for (entity, position, opt_title) in query.iter_mut() {
        if position.x.abs() > CLEANUP_BOUNDS || position.y.abs() > CLEANUP_BOUNDS {
            // Remove any entity that has gone beyond the cleanup bounds.
            match opt_title {
                Some(title) => debug!("Cleaning up: {}: {:?}", title, entity),
                None => debug!("Cleaning up: {:?}", entity),
            }
            commands.entity(entity).despawn();
        }
    }
}
