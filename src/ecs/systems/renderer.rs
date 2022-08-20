use bevy::prelude::{Query, Transform};

use crate::ecs::components::position::Position;

pub fn transform_positions(mut sprite_position: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in sprite_position.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;
    }
}
