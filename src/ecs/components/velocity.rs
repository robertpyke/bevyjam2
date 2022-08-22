use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct Velocity {
    pub magnitude: f32,
    pub angle_radians: f32,
}

impl std::fmt::Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "({}@∠{}°)",
            self.magnitude,
            self.angle_radians.to_degrees()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::ecs::components::velocity::Velocity;

    #[test]
    fn fmt() {
        let v = Velocity {
            magnitude: 12.31,
            angle_radians: PI / 2.,
        };
        assert_eq!(format!("{}", v), "(12.31@∠90°)");
    }
}
