use bevy::prelude::Component;

use super::resource::ResourceType;

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Eq, PartialEq, Copy)]
pub enum CellStructure {
    Producer(ResourceType),
    Collector(ResourceType),
}
