use bevy::prelude::Component;

use crate::ecs::components::resource::ResourceType;

#[derive(Component, Debug, Clone)]
pub struct Consumable {
    pub resource: ResourceType,
    // amount provided when consumed
    pub volume: u32,
}
