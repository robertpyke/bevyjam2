use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct AngularVelocity {
    pub val: f32,
}

impl std::fmt::Display for AngularVelocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "(ω: {})", self.val)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::ecs::components::angular_velocity::AngularVelocity;

    #[test]
    fn fmt() {
        let v = AngularVelocity { val: PI };
        assert_eq!(format!("{}", v), "(ω: 3.1415927)");
    }
}
