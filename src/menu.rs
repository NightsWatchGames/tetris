use std::sync::LazyLock;
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

static MENU_ROOT_NODE: LazyLock<Node> = LazyLock::new(|| {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
});

static MENU_BUTTON_CONTAINER_NODE: LazyLock<Node> = LazyLock::new(|| {
    Node {
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::Center,
        ..default()
    }
});

fn menu_button(text: &str, action: MenuButtonAction) -> impl Bundle {
    (
        Button,
        Node {
            width: Val::Px(120.0),
            height: Val::Px(30.0),
            margin: UiRect::all(Val::Px(10.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.15, 0.15, 0.15).into()),
        action,
        children![(
            Text::new(text),
            TextFont {
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
        )],
    )
}

pub fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        MENU_ROOT_NODE.clone(),
        OnMainMenuScreen,
        children![(
            MENU_BUTTON_CONTAINER_NODE.clone(),
            BackgroundColor(palettes::css::CRIMSON.into()),
            children![
                // 标题
                (
                    Text::new("Tetris Main Menu"),
                    TextFont {
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                ),
                // 开始按钮
                menu_button("Start", MenuButtonAction::StartGame),
                // 退出按钮
                menu_button("Quit", MenuButtonAction::Quit),
            ]
        )],
    ));
}

pub fn setup_game_over_menu(mut commands: Commands) {
    commands.spawn((
        MENU_ROOT_NODE.clone(),
        OnGameOverMenuScreen,
        children![(
            MENU_BUTTON_CONTAINER_NODE.clone(),
            BackgroundColor(palettes::css::CRIMSON.into()),
            children![
                // 标题
                (
                    Text::new("Game Over"),
                    TextFont {
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                ),
                // 返回主菜单按钮
                menu_button("Main Menu", MenuButtonAction::BackToMainMenu),
                // 重新开始按钮
                menu_button("Restart", MenuButtonAction::RestartGame),
            ]
        )],
    ));
}

pub fn setup_game_paused_menu(mut commands: Commands) {
    commands.spawn((
        MENU_ROOT_NODE.clone(),
        OnGamePausedMenuScreen,
        children![(
            MENU_BUTTON_CONTAINER_NODE.clone(),
            BackgroundColor(palettes::css::CRIMSON.into()),
            children![
                // 标题
                (
                    Text::new("Game Paused"),
                    TextFont {
                        font_size: 25.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    Node {
                        margin: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                ),
                // 返回主菜单按钮
                menu_button("Main Menu", MenuButtonAction::BackToMainMenu),
                // 重新开始按钮
                menu_button("Restart", MenuButtonAction::RestartGame),
                // 恢复游戏按钮
                menu_button("Resume", MenuButtonAction::ResumeGame)
            ]
        )],
    ));
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
                    exit.write_default();
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
        commands.entity(entity).despawn();
    }
}
