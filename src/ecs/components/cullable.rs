use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct Cullable {
    pub ticks_remaining: i32,
}
