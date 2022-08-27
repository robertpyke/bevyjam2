use bevy::prelude::Component;

use crate::ecs::components::resource::ResourceType;

#[derive(Component, Debug, Clone)]
pub struct Consumer {
    pub target_resource: ResourceType,
    pub consumption_radius: f32,
    // amount consumed
    pub volume: f32,
}
