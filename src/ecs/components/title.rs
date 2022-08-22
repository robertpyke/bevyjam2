use std::fmt;

use bevy::prelude::Component;

#[derive(Component, Debug, Clone)]
pub struct Title {
    pub val: String,
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use crate::ecs::components::title::Title;

    #[test]
    fn fmt() {
        let t = Title {
            val: "abc".to_string(),
        };
        assert_eq!(format!("{}", t), "abc");
    }
}
