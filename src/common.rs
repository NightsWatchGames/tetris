use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    InGame,
    GameOver,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    GamePlaying,
    GamePaused,
    GameRestarted,
    GameQuitted,
}