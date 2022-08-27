use bevy::prelude::Component;

use crate::ecs::components::cell_structure::CellStructure;

#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum ResourceType {
    Energy,
    Cell(CellStructure),
}
