mod ecs;

use bevy::prelude::*;
use ecs::systems;

fn main() {
    App::new().add_system(systems::example::hello_world).run();
}
