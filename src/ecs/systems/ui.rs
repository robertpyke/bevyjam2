use bevy::prelude::*;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
