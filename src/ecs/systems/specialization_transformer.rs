use crate::ecs::components::{
    cell_structure::CellStructure, consumer::Consumer, gravitational_pull::GravitationalPull,
    position::Position, resource::ResourceType, specialization::Specialization, title::Title,
};
use bevy::{
    prelude::{
        debug, default, shape, Assets, Color, Commands, Entity, Mesh, Query, ResMut, Transform,
        Vec3, With,
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};
use uuid::Uuid;

const GRID_SIZE: f32 = 10.;

pub fn specialization_transformer_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    cell_positions: Query<&Position, With<CellStructure>>,
    consumer_with_specialization_query: Query<(Entity, &Position, &Specialization, &Consumer)>,
) {
    for (entity, position, specialization, consumer) in &consumer_with_specialization_query {
        if consumer.target_resource == specialization.resource_type_required
            && consumer.volume >= specialization.resource_volume_required
        {
            // Removed the consumer and the specialization from the entity
            commands
                .entity(entity)
                .remove::<Consumer>()
                .remove::<Specialization>()
                .insert(specialization.specialization_target);

            for x in [position.x - GRID_SIZE, position.x + GRID_SIZE] {
                for y in [position.y - GRID_SIZE, position.y + GRID_SIZE] {
                    let id = Uuid::new_v4();
                    let z: f32 = 0.;

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
                    let radius = GRID_SIZE / 2.;
                    let gravitational_pull = 10.;

                    // Spawn a consumer next to me..
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
                            specialization_target: CellStructure::Producer(ResourceType::Light),
                            resource_volume_required: 10,
                            resource_type_required: ResourceType::Energy,
                        })
                        .insert(GravitationalPull {
                            mass: gravitational_pull,
                        })
                        .insert_bundle(MaterialMesh2dBundle {
                            mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                            material: materials.add(ColorMaterial::from(Color::GREEN)),
                            transform: Transform::from_translation(Vec3::new(
                                scaled_x, scaled_y, z,
                            )),
                            ..default()
                        });
                }
            }
        }
    }
}
