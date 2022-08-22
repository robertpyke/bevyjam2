use bevy::prelude::Query;

use crate::ecs::components::{position::Position, velocity::Velocity};

pub fn move_system(mut query: Query<(&mut Position, &Velocity)>) {
    for (mut position, velocity) in query.iter_mut() {
        let x_move = velocity.magnitude * velocity.angle_radians.cos();
        let y_move = velocity.magnitude * velocity.angle_radians.sin();

        position.x += x_move;
        position.y += y_move;
    }
}
