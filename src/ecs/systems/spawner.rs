use rand::prelude::*;

use bevy::{
    prelude::{default, shape, Assets, Color, Commands, Mesh, ResMut, Transform, Vec3},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::ecs::components::{position::Position, velocity::Velocity};

const END_OF_WORLD_X: f32 = 100.;
const END_OF_WORLD_Y: f32 = 100.;
const GRID_SIZE: f32 = 10.;

pub fn spawn_background_world_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _i in 0..rng.gen_range(0..3) {
        let x = END_OF_WORLD_X;
        let y = rng.gen_range(-END_OF_WORLD_Y..END_OF_WORLD_Y);
        let z: f32 = 0.;
        let position = Position { x, y }.snap_to_grid(GRID_SIZE);
        let scaled_x = position.x;
        let scaled_y = position.y;
        commands
            .spawn()
            .insert(position)
            .insert(Velocity {
                magnitude: 10.,
                angle_deg: rng.gen_range(0.0..360.),
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(GRID_SIZE / 2.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(scaled_x, scaled_y, z)),
                ..default()
            });
    }
}
