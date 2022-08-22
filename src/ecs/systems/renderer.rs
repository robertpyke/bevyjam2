use bevy::prelude::{Query, Transform};

use crate::ecs::components::{angular_velocity::AngularVelocity, position::Position};

pub fn transform_positions(
    mut sprite_position: Query<(&Position, Option<&AngularVelocity>, &mut Transform)>,
) {
    for (position, opt_angular_velocity, mut transform) in sprite_position.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
        if let Some(angular_velocity) = opt_angular_velocity {
            transform.rotate_z(angular_velocity.val)
        }
    }
}
