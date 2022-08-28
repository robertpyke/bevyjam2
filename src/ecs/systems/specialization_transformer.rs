use std::f32::consts::PI;

use crate::ecs::components::{
    cell_structure::CellStructure, consumer::Consumer, gravitational_pull::GravitationalPull,
    position::Position, resource::ResourceType, specialization::Specialization, title::Title,
};
use bevy::{
    prelude::{
        debug, default, shape, Assets, Commands, Entity, Mesh, Quat, Query, ResMut, Transform,
        Vec3, With,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use rand::seq::SliceRandom;
use uuid::Uuid;

const GRID_SIZE: f32 = 10.;
const RADIUS: f32 = GRID_SIZE / 2.;
const Z: f32 = 0.;

pub fn specialization_transformer_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_positions: Query<&Position, With<CellStructure>>,
    consumer_with_specialization_query: Query<(Entity, &Position, &Specialization, &Consumer)>,
) {
    let mut rng = rand::thread_rng();
    for (entity, position, specialization, consumer) in &consumer_with_specialization_query {
        if consumer.target_resource == specialization.resource_type_required
            && consumer.volume >= specialization.resource_volume_required
        {
            // Removed the consumer and the specialization from the entity
            commands
                .entity(entity)
                .remove::<Consumer>()
                .remove::<Specialization>()
                .insert(specialization.specialization_target)
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(
                        specialization.specialization_target.color(),
                    )),
                    transform: Transform::from_translation(Vec3::new(position.x, position.y, Z)),
                    ..default()
                });

            for x in [position.x - GRID_SIZE, position.x + GRID_SIZE] {
                for y in [position.y - GRID_SIZE, position.y + GRID_SIZE] {
                    let id = Uuid::new_v4();

                    let position = Position { x, y }.snap_to_grid(GRID_SIZE);
                    let &skip = &cell_positions
                        .iter()
                        .any(|existing_cell_position| &position == existing_cell_position);
                    if skip {
                        debug!("Skipping creation as cell exists at: {}", position);
                        continue;
                    }
                    let scaled_x = position.x;
                    let scaled_y = position.y;
                    let gravitational_pull = 10.;

                    let resource_type = [
                        ResourceType::Light,
                        ResourceType::Water,
                        ResourceType::Energy,
                    ]
                    .choose(&mut rng)
                    .unwrap();

                    let mut consume_type = [
                        ResourceType::Light,
                        ResourceType::Water,
                        ResourceType::Energy,
                    ]
                    .choose(&mut rng)
                    .unwrap();
                    while consume_type == resource_type {
                        consume_type = [
                            ResourceType::Light,
                            ResourceType::Water,
                            ResourceType::Energy,
                        ]
                        .choose(&mut rng)
                        .unwrap();
                    }

                    // Spawn a consumer next to me..
                    commands
                        .spawn()
                        .insert(position)
                        .insert(Title {
                            val: format!("consumer: '{}'", id),
                        })
                        .insert(CellStructure::Collector(*consume_type))
                        .insert(Consumer {
                            target_resource: *consume_type,
                            consumption_radius: 5.0,
                            volume: 0,
                        })
                        .insert(Specialization {
                            specialization_target: CellStructure::Producer(*resource_type),
                            resource_volume_required: 10,
                            resource_type_required: *consume_type,
                        })
                        .insert(GravitationalPull {
                            mass: gravitational_pull,
                        })
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Cube::new(GRID_SIZE).into()).into(),
                            material: materials.add(ColorMaterial::from(consume_type.color())),
                            transform: Transform::from_translation(Vec3::new(
                                scaled_x, scaled_y, Z,
                            ))
                            .with_rotation(Quat::from_rotation_z(PI / 4.)),
                            ..default()
                        });
                }
            }
        }
    }
}
