use crate::ecs::components::{consumable::Consumable, consumer::Consumer, position::Position};
use bevy::prelude::{debug, Commands, Entity, Query, Without};
use std::collections::HashSet;

pub fn consumption_system(
    mut commands: Commands,
    mut consumer_query: Query<(Entity, &Position, &mut Consumer, Without<Consumable>)>,
    consumable_query: Query<(Entity, &Position, &Consumable, Without<Consumer>)>,
) {
    // don't double process consumed entities
    let mut consumed = HashSet::new();

    for (consumer_entity, consumer_position, mut consumer, _) in consumer_query.iter_mut() {
        for (consumable_entity, consumable_position, consumable, _) in &consumable_query {
            if !consumed.contains(&consumable_entity.id())
                && (consumer.target_resource == consumable.resource)
            {
                let x_dist = (consumable_position.x - consumer_position.x).abs();
                let y_dist = (consumable_position.y - consumer_position.y).abs();
                let hyp_dist = (x_dist.powi(2) + y_dist.powi(2)).sqrt();
                if hyp_dist < consumer.consumption_radius {
                    consumer.volume += consumable.volume;
                    debug!("{:?} consumed {:?}", consumer_entity, consumable_entity);
                    consumed.insert(consumable_entity.id());
                    commands.entity(consumable_entity).despawn();
                }
            }
        }
    }
}
