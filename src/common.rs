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

#[derive(Debug, Resource)]
pub struct GameAudios {
    pub drop: Handle<AudioSource>,
    pub gameover: Handle<AudioSource>,
    pub line_clear: Handle<AudioSource>,
}

pub fn setup_game_audios(mut command: Commands, asset_server: Res<AssetServer>) {
    let game_audios = GameAudios {
        drop: asset_server.load("sounds/Drop.wav"),
        gameover: asset_server.load("sounds/Gameover.wav"),
        line_clear: asset_server.load("sounds/Lineclear.wav"),
    };
    command.insert_resource(game_audios);
}
