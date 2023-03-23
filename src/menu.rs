use bevy::app::AppExit;
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

// 拆封为插件，分离代码
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // 监控状态创建界面
        app.add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(setup_game_over_menu.in_schedule(OnEnter(AppState::GameOver)))
            .add_system(setup_game_paused_menu.in_schedule(OnEnter(GameState::GamePaused)));

        // TODO 检查菜单按钮系统 这俩也可以用run_if 一个方法来判断是否运行
        app.add_system(
            click_button.run_if(
                state_exists_and_equals(AppState::MainMenu)
                    .or_else(state_exists_and_equals(AppState::GameOver))
                    .or_else(state_exists_and_equals(GameState::GamePaused)),
            ),
        );
        // 按下ESC暂停 再次按下恢复
        app.add_system(
            pause_game.run_if(
                state_exists_and_equals(GameState::GamePlaying)
                    .or_else(state_exists_and_equals(GameState::GamePaused)),
            ),
        );

        // 重新开始游戏
        app.add_system(play_game.in_set(OnUpdate(GameState::GameRestarted)));

        // 销毁界面
        app.add_system(despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(
                despawn_screen::<OnGameOverMenuScreen>.in_schedule(OnExit(AppState::GameOver)),
            )
            .add_system(
                despawn_screen::<OnGamePausedMenuScreen>.in_schedule(OnExit(GameState::GamePaused)),
            );
    }
}

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Tetris Main Menu",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
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
                                    size: Size::new(Val::Px(50.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::StartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    // 退出按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(50.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Over",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
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
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    // 重新开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::RestartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Restart",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

fn setup_game_paused_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    background_color: Color::CRIMSON.into(),
                    ..default()
                })
                .with_children(|parent| {
                    // 标题
                    parent.spawn(
                        TextBundle::from_section(
                            "Game Paused",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 25.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
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
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Main Menu",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    // 重新开始按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::RestartGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Restart",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });

                    // 恢复游戏按钮
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(90.0), Val::Px(30.0)),
                                    margin: UiRect::all(Val::Px(10.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..default()
                            },
                            MenuButtonAction::ResumeGame,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Resume",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                });
        });
}

fn click_button(
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
            Interaction::Clicked => match menu_button_action {
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
                    println!("{:?}", app_state.0);
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

fn pause_game(
    game_state: Res<State<GameState>>,
    mut change_game_state: ResMut<NextState<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // 修改为可以再次按下ESC恢复游戏，使用just_pressed防止循环触发
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if let GameState::GamePlaying = game_state.0 {
            change_game_state.set(GameState::GamePaused);
        } else {
            change_game_state.set(GameState::GamePlaying);
        }
    }
}

fn play_game(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::GamePlaying);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
