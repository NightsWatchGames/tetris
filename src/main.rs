use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use board::*;
use piece::*;
use stats::*;

mod board;
mod piece;
mod stats;

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .insert_resource(Score(0))
        .insert_resource(Lines(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_board)
        .add_startup_system(setup_stats_boards)
        .add_system(update_scoreboard)
        .add_system(update_linesboard)
        .add_system_set(
            SystemSet::new()
                .with_system(check_collision)
                .with_system(manually_move_piece.after(check_collision))
                .with_system(auto_move_piece_down.after(check_collision))
        )
        .add_system(auto_generate_new_piece)
        .add_system(check_line)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
