use bevy::prelude::*;

use crate::{board::*, common::AppState};

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

pub fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Show main menu");
    let button_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // center button
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
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
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();
}

pub fn setup_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Show game over menu");
    let button_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    // center button
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            OnGameOverMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(200.0), Val::Px(65.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
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
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();
}

pub fn click_button(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<AppState>>,
) {
    for (interaction, menu_button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match menu_button_action {
                MenuButtonAction::StartGame => {
                    info!("StartGame button clicked");
                    state.set(AppState::GamePlaying).unwrap()
                }
                MenuButtonAction::RestartGame => {
                    info!("RestartGame button clicked");
                    state.set(AppState::MainMenu).unwrap()
                }
                MenuButtonAction::BackToMainMenu => {
                    info!("BackToMainMenu button clicked");
                    state.set(AppState::MainMenu).unwrap()
                }
                MenuButtonAction::ResumeGame => {
                    info!("ResumeGame button clicked");
                    state.set(AppState::GamePlaying).unwrap()
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
