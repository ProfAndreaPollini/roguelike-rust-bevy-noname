use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers};
use bevy_ecs_tilemap::tiles::TilePos;
use bevy_inspector_egui::egui::Margin;

use crate::{
    events::TurnEndEvent, resources::RLTimeSystem, ButtonStatus, GameUiCamera, MyAssets,
    MyGameCamera, Player, PlayerPositionUILabel, TimeUIButton, TimeUIField,
};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn game_ui_setup(mut commands: Commands, assets: Res<MyAssets>) {
    let bg_color: Color = Color::hex("1d181688").unwrap();
    let text_color = Color::hex("fcfcfc").unwrap();
    let data_color = Color::hex("f7d8bc").unwrap();
    // // ui camera
    commands.spawn((
        Camera2dBundle {
            // transform: Transform::from_xyz(10.0, 10., -5.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::None,
            },
            camera: Camera {
                // renders after / on top of the main camera
                order: 1,
                ..Default::default()
            },
            ..default()
        },
        GameUiCamera {},
        UiCameraConfig { show_ui: true },
        RenderLayers::from_layers(&[1]),
    ));

    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                flex_grow: 1.0,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                left: Val::Percent(70.),

                margin: UiRect {
                    left: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: BackgroundColor(bg_color),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            top: Val::Px(10.),
                            ..Default::default()
                        },
                        width: Val::Percent(100.),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                })
                .with_children(|builder| {
                    text_row(
                        builder,
                        "Hello, world!",
                        TextStyle {
                            font: assets.ui_font.clone(),
                            font_size: 48.0,
                            color: text_color,
                        },
                    );
                    text_row(
                        builder,
                        "Hello, world data!",
                        TextStyle {
                            font: assets.ui_font.clone(),
                            font_size: 28.0,
                            color: data_color,
                        },
                    );

                    builder.spawn((
                        TextBundle::from_sections([
                            TextSection::new(
                                "player position: 0, 0",
                                TextStyle {
                                    font: assets.ui_font.clone(),
                                    font_size: 22.0,
                                    color: data_color,
                                },
                            ),
                            TextSection::new(
                                "Hello, ",
                                TextStyle {
                                    font: assets.ui_font.clone(),
                                    font_size: 22.0,
                                    color: data_color,
                                },
                            ),
                        ]),
                        PlayerPositionUILabel::default(),
                    ));
                    builder.spawn((
                        TextBundle::from_section(
                            "time: 00:00:00",
                            TextStyle {
                                font: assets.ui_font.clone(),
                                font_size: 22.0,
                                color: data_color,
                            },
                        ),
                        TimeUIField::default(),
                    ));
                    builder
                        .spawn((
                            ButtonBundle {
                                background_color: BackgroundColor(
                                    Color::hex("193c3eff").unwrap_or(bg_color),
                                ),

                                style: Style {
                                    margin: UiRect {
                                        top: Val::Px(10.),
                                        ..Default::default()
                                    },

                                    ..Default::default()
                                },

                                ..Default::default()
                            },
                            TimeUIButton::default(),
                            ButtonStatus::default(),
                        ))
                        .with_children(|parent| {
                            parent.spawn((TextBundle {
                                text: Text {
                                    sections: vec![TextSection {
                                        value: "Button".to_string(),
                                        style: TextStyle {
                                            font: assets.ui_font.clone(),
                                            font_size: 22.0,
                                            color: text_color,
                                        },
                                    }],
                                    ..Default::default()
                                },
                                ..Default::default()
                            },));
                        });
                });
            // builder.spawn(NodeBundle {
            //     style: Style {
            //         flex_direction: FlexDirection::Row,
            //         margin: UiRect {
            //             top: Val::Px(10.),
            //             ..Default::default()
            //         },
            //         width: Val::Percent(100.),
            //         height: Val::Px(100.),
            //         ..Default::default()
            //     },
            //     background_color: BackgroundColor(Color::RED),
            //     ..Default::default()
            // });
        });
}

fn text_row(builder: &mut ChildBuilder, text: &str, style: TextStyle) {
    builder.spawn(TextBundle::from_section(text, style));
}

pub fn game_ui_interaction(
    mut btn_query: Query<
        (&Interaction, &mut BackgroundColor, &mut ButtonStatus),
        (With<TimeUIButton>),
    >,
    mut end_turn_ew: EventWriter<TurnEndEvent>,
) {
    let hover_color = Color::hex("265c42ff").unwrap();
    let normal_bg_color = Color::hex("193c3eff").unwrap();

    let (interaction, mut bg_color, mut status) = match btn_query.get_single_mut() {
        Ok((interaction, mut bg_color, mut status)) => (interaction, bg_color, status),
        Err(_) => return,
    };
    match (*interaction, *status) {
        (Interaction::Pressed, ButtonStatus::Normal | ButtonStatus::Hovered) => {
            bg_color.0 = PRESSED_BUTTON;
            end_turn_ew.send(TurnEndEvent {});
            println!("Pressed");
            *status = ButtonStatus::Pressed;
        }
        (Interaction::Hovered, _) => {
            bg_color.0 = hover_color;
            *status = ButtonStatus::Hovered;
        }
        (Interaction::None, ButtonStatus::Pressed) => {
            bg_color.0 = normal_bg_color;
            *status = ButtonStatus::Hovered;
        }
        (Interaction::None, ButtonStatus::Hovered) => {
            bg_color.0 = normal_bg_color;
            *status = ButtonStatus::Normal;
        }
        _ => {
            bg_color.0 = NORMAL_BUTTON;
        }
    }
}

pub fn game_ui_update(
    mut query: Query<&mut Text, With<TimeUIField>>,
    rl_time: Res<RLTimeSystem>,
    // mut player_position_query: Query<(&Transform), (With<Player>)>,
    // mut player_position_label_query: Query<(&mut Text), (With<PlayerPositionUILabel>)>,
    btn_query: Query<(&ButtonStatus), (With<TimeUIButton>)>,
) {
    let btn_status = match btn_query.get_single() {
        Ok(btn) => btn,
        Err(_) => return,
    };
    let mut text = match query.get_single_mut() {
        Ok(btn) => btn,
        Err(_) => return,
    };

    // let mut player_position_text = match player_position_label_query.get_single_mut() {
    //     Ok(position_label) => position_label,
    //     Err(_) => return,
    // };

    text.sections[0].value = format!("{} ({:?})", *rl_time, btn_status);
    // player_position_text.sections[0].value = format!(
    //     "player position: {:?}",
    //     player_position_query
    //         .get_single_mut()
    //         .map(|transform| transform.translation.truncate())
    // );
}

pub fn game_ui_player_position_update(
    player_position_query: Query<(&Transform, &GlobalTransform, &TilePos), With<Player>>,
    mut player_position_label_query: Query<&mut Text, With<PlayerPositionUILabel>>,
    camera_q: Query<(&GlobalTransform, &Camera), (With<MyGameCamera>, Without<Player>)>,
) {
    let mut player_position_text = match player_position_label_query.get_single_mut() {
        Ok(position_label) => position_label,
        Err(_) => return,
    };

    if let Ok((transform, global_transform, tile_pos)) = player_position_query.get_single() {
        player_position_text.sections[0].value = format!(
            "player position: {:?} [ tile: {:?}] global: {:?}",
            transform.translation.truncate(),
            tile_pos,
            global_transform.translation().truncate()
        );
    }

    if let Ok((global_transform, camera)) = camera_q.get_single() {
        player_position_text.sections[1].value = format!(
            "camnera position: {:?} [ camera: {:?}]",
            global_transform.translation().truncate(),
            camera
        );
    }
}
