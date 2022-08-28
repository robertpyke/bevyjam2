use bevy::prelude::Component;

use crate::ecs::components::resource::ResourceType;

use super::cell_structure::CellStructure;

// Simplified to single resource required
#[derive(Component, Debug, Clone)]
pub struct Specialization {
    pub specialization_target: CellStructure,
    pub resource_volume_required: u32,
    pub resource_type_required: ResourceType,
}
