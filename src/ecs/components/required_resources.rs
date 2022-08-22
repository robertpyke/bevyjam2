use bevy::prelude::Component;

use crate::ecs::components::cell_structure::CellStructure;

#[derive(Component, Debug, Clone)]
pub struct RequiredResources {
    pub required_energy: u16,
    pub target_structure: CellStructure,
}

impl RequiredResources {
    pub fn new(target_structure: CellStructure) -> Self {
        RequiredResources {
            required_energy: target_structure.get_required_energy(),
            target_structure,
        }
    }

    pub fn decrement_required_energy(&mut self) {
        if self.required_energy > 0 {
            self.required_energy -= 1;
        }
    }
}

impl std::fmt::Display for RequiredResources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}
