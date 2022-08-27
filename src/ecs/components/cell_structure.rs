use bevy::prelude::Component;

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum CellStructure {
    Producer,
    Collector,
}
