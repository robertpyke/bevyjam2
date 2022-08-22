use std::f32::consts::PI;

use rand::prelude::*;
use uuid::Uuid;

use bevy::{
    prelude::{default, shape, Assets, Color, Commands, Mesh, ResMut, Transform, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::ecs::components::{
    angular_velocity::AngularVelocity, position::Position, title::Title, velocity::Velocity,
};

const END_OF_WORLD_X: f32 = 450.;
const END_OF_WORLD_Y: f32 = 450.;
const GRID_SIZE: f32 = 10.;
const SPAWN_UP_TO: u8 = 10;

pub fn spawn_background_world_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _i in 0..rng.gen_range(0..SPAWN_UP_TO) {
        let id = Uuid::new_v4();
        let x = END_OF_WORLD_X;
        let y = rng.gen_range(-END_OF_WORLD_Y..END_OF_WORLD_Y);
        let z: f32 = 0.;
        let position = Position { x, y }.snap_to_grid(GRID_SIZE);
        let scaled_x = position.x;
        let scaled_y = position.y;
        let size = rng.gen_range((GRID_SIZE / 2.)..(GRID_SIZE * 2.));
        commands
            .spawn()
            .insert(position)
            .insert(Title {
                val: format!("floater: '{}'", id),
            })
            .insert(AngularVelocity {
                val: rng.gen_range(-PI..PI),
            })
            .insert(Velocity {
                magnitude: rng.gen_range(1.0..10.),
                angle_radians: PI,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(size).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(scaled_x, scaled_y, z)),
                ..default()
            });
    }
}
