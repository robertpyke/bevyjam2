use std::f32::consts::PI;

use bevy::prelude::{Entity, Query};

use crate::ecs::components::{
    gravitational_pull::GravitationalPull, position::Position, velocity::Velocity,
};

pub fn move_system(
    mut query: Query<(
        Entity,
        &mut Position,
        Option<&mut Velocity>,
        Option<&GravitationalPull>,
    )>,
) {
    let v: Vec<(u32, Position, GravitationalPull)> = query
        .iter()
        .filter(|(_entity, _position, _velocity, gravitation_pull)| gravitation_pull.is_some())
        .map(|(entity, position, _velocity, gravitation_pull)| {
            let pull = gravitation_pull.unwrap_or(&GravitationalPull { mass: 0. });
            (entity.id(), position.clone(), pull.clone())
        })
        .collect();
    for (entity, position, velocity_opt, _) in query.iter_mut() {
        if velocity_opt.is_none() {
            continue;
        }
        let mut velocity = velocity_opt.unwrap();
        for (gravity_entity_id, gravity_position, gravitation_pull) in &v {
            if gravitation_pull.mass <= 0. {
                continue;
            }
            // Skip gravity calculation if we're looking at ourselves.
            if &entity.id() == gravity_entity_id {
                continue;
            }

            let x_dist = position.x - gravity_position.x;
            let y_dist = position.y - gravity_position.y;
            let angle_to_gravity = if x_dist == 0. && y_dist == 0. {
                0.
            } else {
                ((y_dist / x_dist) as f32).atan()
            };

            let hyp = (x_dist.abs().powi(2) + y_dist.abs().powi(2)).sqrt();
            // Ensure gravity is never too extreme
            let hyp = f32::max(5., hyp);
            let b = velocity.angle_radians - angle_to_gravity;
            let b = (b + PI) % (2. * PI) - PI;

            // Move a fractional amount of the desired angle, based on the distance away, and the gravitational pull
            velocity.angle_radians += (b / hyp) * (gravitation_pull.mass / 10.);
        }
    }

    for (_entity, mut position, velocity, _gravity_position) in query.iter_mut() {
        if velocity.is_none() {
            continue;
        }
        let velocity = velocity.unwrap();
        let x_move = velocity.magnitude * velocity.angle_radians.cos();
        let y_move = velocity.magnitude * velocity.angle_radians.sin();

        position.x += x_move;
        position.y += y_move;
    }
}
