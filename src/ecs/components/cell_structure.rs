use bevy::prelude::{Color, Component};

use super::resource::ResourceType;

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Eq, PartialEq, Copy)]
pub enum CellStructure {
    Producer(ResourceType),
    Collector(ResourceType),
}

impl CellStructure {
    pub fn color(&self) -> Color {
        match self {
            CellStructure::Producer(resource_type) => resource_type.color(),
            CellStructure::Collector(resource_type) => resource_type.color(),
        }
    }
}
