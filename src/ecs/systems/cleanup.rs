use bevy::prelude::{debug, Commands, Entity, Query};

use crate::ecs::components::{cullable::Cullable, position::Position, title::Title};

const CLEANUP_BOUNDS: f32 = 4000.;

pub fn cleanup_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Position, &Cullable, Option<&Title>)>,
) {
    for (entity, position, cullable, opt_title) in query.iter_mut() {
        if position.x.abs() > CLEANUP_BOUNDS
            || position.y.abs() > CLEANUP_BOUNDS
            || cullable.ticks_remaining <= 0
        {
            // Remove any entity that has gone beyond the cleanup bounds.
            match opt_title {
                Some(title) => debug!("Cleaning up: {}: {:?}", title, entity),
                None => debug!("Cleaning up: {:?}", entity),
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn cleanup_tick(mut query: Query<&mut Cullable>) {
    for mut cullable in query.iter_mut() {
        cullable.ticks_remaining -= 1;
    }
}
