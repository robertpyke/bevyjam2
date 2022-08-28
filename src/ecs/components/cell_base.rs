use bevy::prelude::Component;

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Eq, PartialEq)]
pub enum CellBase {
    Stem,
    Growth,
    Plant,
    Carnoivore,
    Herbivore,
}
