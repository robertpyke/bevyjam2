use crate::ecs::components::{
    cell_base::CellBase, cell_structure::CellStructure, menu_open::MenuOpen, position::Position,
};
use bevy::{input::mouse::MouseButtonInput, input::ButtonState, prelude::*};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    let font = asset_server.load("fonts/PressStart2P.ttf");
    let instruction_text_style = TextStyle {
        font: font.clone(),
        font_size: 8.0,
        color: Color::WHITE,
    };
    let instruction_base_style: Style = Style {
        padding: UiRect::all(Val::Px(5.)),
        margin: UiRect::all(Val::Px(5.)),
        size: Size::new(Val::Px(180.), Val::Auto),
        ..default()
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexEnd,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
            })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Undefined, Val::Undefined),
                        flex_grow: 1.,
                        align_items: AlignItems::FlexEnd,
                        margin: UiRect::all(Val::Px(10.)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(
                            TextBundle::from_section(
                                "Mitosis",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                    font: font.clone()
                                }
                            )
                            .with_style( Style {
                                ..default()
                            }),
                        );
                });
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        size: Size::new(Val::Px(200.), Val::Auto),
                        padding: UiRect::all(Val::Px(5.)),
                        justify_content: JustifyContent::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Objective: Gather, Process, Build, Grow",
                            instruction_text_style.clone()
                        )
                        .with_style(instruction_base_style.clone())
                    );
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Instructions:",
                            instruction_text_style.clone()
                        )
                        .with_style(instruction_base_style.clone())
                    );
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Grey stem cells will appear on the border of your colony. These cells can either grow the current cell or specialize into another cell type",
                            instruction_text_style.clone()
                        )
                        .with_style(instruction_base_style.clone())
                    );
                    parent.spawn_bundle(
                        TextBundle::from_section(
                            "Click on specialized cell segment to see available cellular machinery.",
                            instruction_text_style.clone()
                        )
                        .with_style(instruction_base_style.clone())
                    );
                });
        });
}

// system to render a ui element that displays current mouse coordinates
pub fn mouse_position_ui(windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    let cursor_position = window.cursor_position();
    if cursor_position.is_none() {
        return;
    }
    let cursor_position = cursor_position.unwrap();
    // println!("Mouse position: {:?}", cursor_position);
}

pub fn spawn_menus(
    mut commands: Commands,
    query: Query<(Entity, &Position, With<CellBase>, &MenuOpen)>,
    asset_server: Res<AssetServer>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    for query_entry in query.iter() {
        let position = query_entry.1;
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Px(position.x as f32 + (&window.width() / 2.0)),
                        top: Val::Px(
                            &window.height() - (position.y as f32 + (&window.height() / 2.0)),
                        ),
                        ..Default::default()
                    },
                    size: Size::new(Val::Auto, Val::Auto),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Auto),
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(TextBundle::from_section(
                            format!("Cell Clicked"),
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                font: asset_server.load("fonts/PressStart2P.ttf"),
                            },
                        ));
                    });
            });
        commands.entity(query_entry.0).remove::<MenuOpen>();
    }
}

// system that checks whether left mouse button has been clicked,
// then checks the position of the mouse.  if the position of the mouse is located near an
// entity that has a CellBase component and a position, it adds the MenuOpen component to that
// entity.
pub fn set_open_menu_flag(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    query: Query<(Entity, &Position, With<CellBase>)>,
    windows: Res<Windows>,
) {
    // use bevy::input::ButtonState;
    let window = windows.get_primary().unwrap();
    let cursor_position = window.cursor_position();
    if cursor_position.is_none() {
        return;
    }
    let mut cursor_position = cursor_position.unwrap();
    cursor_position.y = cursor_position.y - window.height() as f32 / 2.;
    cursor_position.x = cursor_position.x - window.width() as f32 / 2.;
    for ev in mousebtn_evr.iter() {
        if ev.button == MouseButton::Left && ev.state == ButtonState::Pressed {
            println!("Left mouse button pressed");
            println!("Mouse position: {:?}", &cursor_position);
            for (entity, position, _) in query.iter() {
                println!("Entity position: {:?}", &position);
                if (&cursor_position.x - &position.x).abs() < 10.0
                    && (&cursor_position.y - &position.y).abs() < 10.0
                {
                    println!("Entity clicked: {:?}", entity);
                    commands.entity(entity).insert(MenuOpen);
                }
            }
        }
    }

    //     if mouse_button_input.just_pressed(MouseButton::Left) {
    //         let window = windows.get_primary().unwrap();
    //         let cursor_position = window.cursor_position();
    //         if cursor_position.is_none() {
    //             return;
    //         }
    //         let cursor_position = cursor_position.unwrap();
    //         for query_entry in query.iter() {
    //             let position = query_entry.1;
    //             println!("{:?}", position);
    //             if (cursor_position.x > position.x && cursor_position.x < position.x + 100.0) && (cursor_position.y > position.y && cursor_position.y < position.y + 100.0) {
    //                 commands.entity(query_entry.0).insert(MenuOpen);
    //             }
    //         }
    //     }
}
