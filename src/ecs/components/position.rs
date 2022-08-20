use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Position {
    fn snap_to_grid(self: &Self, grid_size: f32) -> Position {
        return Position {
            x: (self.x / f32::from(grid_size)).floor() * grid_size,
            y: (self.y / f32::from(grid_size)).floor() * grid_size,
        };
    }
}

// Rust standard practice for unit tests is to put the test in the same file
// but in a tests child module
// https://doc.rust-lang.org/book/ch11-03-test-organization.html

#[cfg(test)]
mod tests {
    use crate::ecs::components::position::Position;

    #[test]
    fn fmt() {
        let position = Position { x: 0.1, y: 0.2 };

        assert_eq!(format!("{}", position), "(0.1, 0.2)");
    }

    #[test]
    fn snap_to_grid_low() {
        let position = Position { x: 0.1, y: 0.2 };

        let snapped_position = position.snap_to_grid(10.);
        assert_eq!(format!("{}", snapped_position), "(0, 0)");
    }

    #[test]
    fn snap_to_grid_high() {
        let position = Position { x: 10.1, y: 4.9 };

        let snapped_position = position.snap_to_grid(5.);
        assert_eq!(format!("{}", snapped_position), "(10, 0)");
    }
}
