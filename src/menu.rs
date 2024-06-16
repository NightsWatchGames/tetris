use bevy::app::AppExit;
use bevy::color::palettes;
use bevy::prelude::*;

use crate::common::{AppState, GameState};

#[derive(Component)]
pub struct OnMainMenuScreen;

#[derive(Component)]
pub struct OnGamePausedMenuScreen;

#[derive(Component)]
pub struct OnGameOverMenuScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    StartGame,
    RestartGame,
    BackToMainMenu,
    ResumeGame,
    Quit,
}

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: palettes::css::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Tetris Main Menu",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // 开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::StartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    // 退出按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

pub fn setup_game_over_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: palettes::css::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Over",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // 返回主菜单按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    // 重新开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::RestartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Restart",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

pub fn setup_game_paused_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnGamePausedMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: palettes::css::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Paused",
                            TextStyle {
                                font_size: 25.0,
                                color: Color::srgb(0.9, 0.9, 0.9),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    // 返回主菜单按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    // 重新开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::RestartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Restart",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });

                    // 恢复游戏按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Px(90.0),
                                    height: Val::Px(30.0),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                image: UiImage::default()
                                    .with_color(Color::srgb(0.15, 0.15, 0.15).into()),
                                ..default()
                            },
                            MenuButtonAction::ResumeGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resume",
                                TextStyle {
                                    font_size: 20.0,
                                    color: Color::srgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ));
                        });
                });
        });
}

pub fn click_button(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match menu_button_action {
                MenuButtonAction::StartGame => {
                    info!("StartGame button clicked");
                    app_state.set(AppState::InGame);
                    game_state.set(GameState::GamePlaying);
                }
                MenuButtonAction::RestartGame => {
                    info!("RestartGame button clicked");
                    app_state.set(AppState::InGame);
                    game_state.set(GameState::GameRestarted);
                }
                MenuButtonAction::BackToMainMenu => {
                    info!("BackToMainMenu button clicked");
                    println!("{:?}", app_state);
                    app_state.set(AppState::MainMenu);
                    game_state.set(GameState::GameQuited);
                }
                MenuButtonAction::ResumeGame => {
                    info!("ResumeGame button clicked");
                    game_state.set(GameState::GamePlaying);
                }
                MenuButtonAction::Quit => {
                    info!("Quit button clicked");
                    exit.send_default();
                }
            },
            _ => {}
        }
    }
}

pub fn pause_game(
    game_state: Res<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // 修改为可以再次按下ESC恢复游戏，使用just_pressed防止循环触发
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let GameState::GamePlaying = game_state.get() {
            change_game_state.set(GameState::GamePaused);
        } else {
            change_game_state.set(GameState::GamePlaying);
        }
    }
}

pub fn play_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::GamePlaying);
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
