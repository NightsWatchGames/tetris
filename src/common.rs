use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    GamePlaying,
    GamePaused,
    GameRestarted,
    #[default]
    GameQuited,
}

#[derive(Debug, Component)]
pub struct LineClearAudioMarker;

#[derive(Debug, Component)]
pub struct GameOverAudioMarker;

#[derive(Debug, Component)]
pub struct DropAudioMarker;

pub fn setup_game_audios(mut command: Commands, asset_server: Res<AssetServer>) {
    let drop = asset_server.load("sounds/Drop.wav");
    let game_over = asset_server.load("sounds/Gameover.wav");
    let line_clear = asset_server.load("sounds/Lineclear.wav");

    command.spawn((
        AudioBundle {
            source: drop,
            ..Default::default()
        },
        DropAudioMarker,
    ));
    command.spawn((
        AudioBundle {
            source: game_over,
            ..Default::default()
        },
        GameOverAudioMarker,
    ));
    command.spawn((
        AudioBundle {
            source: line_clear,
            ..Default::default()
        },
        LineClearAudioMarker,
    ));
}
