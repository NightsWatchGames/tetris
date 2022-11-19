use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use board::*;
use piece::*;
use stats::*;

mod board;
mod piece;
mod stats;

fn main() {
    App::new()
        .insert_resource(Score(0))
        .insert_resource(Lines(0))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_board)
        .add_startup_system(setup_stats_boards)
        .add_startup_system(spawn_piece)
        .add_system(update_scoreboard)
        .add_system(update_linesboard)
        .add_system(move_piece)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
