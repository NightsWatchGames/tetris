use std::time::Duration;

use bevy::prelude::*;
// use bevy_inspector_egui::prelude::*;
use board::*;
use common::*;
use menu::MenuPlugin;
use piece::*;
use stats::*;

mod board;
mod common;
mod menu;
mod piece;
mod stats;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    let mut app = App::new();

    app.insert_resource(Score(0))
        .insert_resource(Lines(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(NextPieceType(None))
        .insert_resource(AutoMovePieceDownTimer(Timer::new(
            Duration::from_millis(1000),
            TimerMode::Repeating,
        )))
        .insert_resource(ManuallyMoveTimer(Timer::new(
            Duration::from_millis(100),
            TimerMode::Once,
        )))
        .insert_resource(RemovePieceComponentTimer(Timer::new(
            Duration::from_millis(300),
            TimerMode::Once,
        )));

    app.add_state::<AppState>().add_state::<GameState>();

    app.add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(StatsPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecePlugin);

    app.add_startup_system(setup_camera).run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
