use crate::ecs::components::{cell_base::CellBase, menu_open::MenuOpen, position::Position};
use bevy::prelude::*;

pub fn open_menu(
    mut commands: Commands,
    query: Query<(Entity, &Position, With<CellBase>)>,
    mouse_buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let cursor_position = windows.get_primary().unwrap().cursor_position();
        if cursor_position.is_none() {
            return;
        }
        let cursor_position = cursor_position.unwrap();
        for (entity, position, _cell_base) in query.iter() {
            let x_dist = position.x - cursor_position.x;
            let y_dist = position.y - cursor_position.y;
            let hyp = (x_dist.abs().powi(2) + y_dist.abs().powi(2)).sqrt();
            if hyp < 10. {
                commands.entity(entity).insert(MenuOpen);
                return;
            }
        }
    }
}
