use bevy::{prelude::*, time::FixedTimestep};
use bevy_inspector_egui::prelude::*;
use board::*;
use piece::*;
use stats::*;
use menu::*;

mod board;
mod piece;
mod stats;
mod menu;

const BACKGROUND_COLOR: Color = Color::BLACK;
const TIME_STEP: f32 = 1.0 / 10.0;

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
        .add_event::<GameOverEvent>()
        .add_system(update_scoreboard)
        .add_system(update_linesboard)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(manually_move_piece)
        )
        .add_system(auto_move_piece_down)
        .add_system_to_stage(CoreStage::PostUpdate, check_collision)
        .add_system(auto_generate_new_piece)
        .add_system(check_line)
        .add_system(rotate_piece)
        .add_system(check_game_over)
        .add_system(game_over_menu)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
