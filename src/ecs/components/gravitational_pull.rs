use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct GravitationalPull {
    pub mass: f32,
}
