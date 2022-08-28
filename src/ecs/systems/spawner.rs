use std::f32::consts::PI;

use rand::prelude::*;
use uuid::Uuid;

use bevy::{
    prelude::{
        default, shape, Assets, Color, Commands, Mesh, Quat, Query, ResMut, Transform, Vec3,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use crate::ecs::components::{
    angular_velocity::AngularVelocity, cell_structure::CellStructure, consumable::Consumable,
    consumer::Consumer, gravitational_pull::GravitationalPull, position::Position,
    resource::ResourceType, specialization::Specialization, title::Title, velocity::Velocity,
};

const END_OF_WORLD_X: f32 = 450.;
const END_OF_WORLD_Y: f32 = 450.;
const GRID_SIZE: f32 = 10.;
const SPAWN_UP_TO: u8 = 10;

pub fn spawn_initial_consumers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _i in 0..rng.gen_range(5..SPAWN_UP_TO) {
        let id = Uuid::new_v4();
        let x = rng.gen_range(-END_OF_WORLD_X..END_OF_WORLD_X);
        let y = rng.gen_range(-END_OF_WORLD_Y..END_OF_WORLD_Y);
        let z: f32 = 0.;
        let position = Position { x, y }.snap_to_grid(GRID_SIZE);
        let scaled_x = position.x;
        let scaled_y = position.y;
        let gravitational_pull = 10.;

        // Spawn a consumer
        commands
            .spawn()
            .insert(position)
            .insert(Title {
                val: format!("consumer: '{}'", id),
            })
            .insert(CellStructure::Collector(ResourceType::Energy))
            .insert(Consumer {
                target_resource: ResourceType::Energy,
                consumption_radius: 5.0,
                volume: 0,
            })
            .insert(Specialization {
                specialization_target: CellStructure::Producer(ResourceType::Water),
                resource_volume_required: 10,
                resource_type_required: ResourceType::Energy,
            })
            .insert(GravitationalPull {
                mass: gravitational_pull,
            })
            .insert_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Cube::new(GRID_SIZE).into()).into(),
                material: materials.add(ColorMaterial::from(ResourceType::Energy.color())),
                transform: Transform::from_translation(Vec3::new(scaled_x, scaled_y, z))
                    .with_rotation(Quat::from_rotation_z(PI / 4.)),
                ..default()
            });
    }
}

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
            .insert(ResourceType::Energy)
            .insert(Consumable {
                resource: ResourceType::Energy,
                volume: 1,
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

pub fn spawn_producer_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(&Position, &CellStructure)>,
) {
    let mut rng = rand::thread_rng();
    for (position, cell_structure) in &query {
        if let CellStructure::Producer(produced_resource) = cell_structure {
            let id = Uuid::new_v4();
            let z: f32 = 0.;
            let size = rng.gen_range((GRID_SIZE / 2.)..(GRID_SIZE));
            commands
                .spawn()
                .insert(position.clone())
                .insert(Title {
                    val: format!("floater: '{}'", id),
                })
                .insert(Consumable {
                    resource: *produced_resource,
                    volume: 1,
                })
                .insert(AngularVelocity {
                    val: rng.gen_range(-PI..PI),
                })
                .insert(Velocity {
                    magnitude: rng.gen_range(2.0..10.0),
                    angle_radians: rng.gen_range(-PI..PI),
                })
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(size).into()).into(),
                    material: materials.add(ColorMaterial::from(produced_resource.color())),
                    transform: Transform::from_translation(Vec3::new(position.x, position.y, z)),
                    ..default()
                });
        }
    }
}
