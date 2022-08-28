use bevy::prelude::{Color, Component};

#[derive(Component, Debug, Clone, Eq, PartialEq, Copy)]
pub enum ResourceType {
    Energy,
    Light,
    Water,
}

impl ResourceType {
    pub fn color(self) -> Color {
        match self {
            ResourceType::Energy => Color::PURPLE,
            ResourceType::Light => Color::YELLOW,
            ResourceType::Water => Color::MIDNIGHT_BLUE,
        }
    }
}
