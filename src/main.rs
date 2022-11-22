use bevy::{prelude::*, time::FixedTimestep};
use bevy_inspector_egui::prelude::*;
use board::*;
use common::*;
use menu::*;
use piece::*;
use stats::*;

mod board;
mod common;
mod menu;
mod piece;
mod stats;

const BACKGROUND_COLOR: Color = Color::BLACK;
const TIME_STEP: f32 = 1.0 / 10.0;

fn main() {
    App::new()
        .insert_resource(Score(0))
        .insert_resource(Lines(0))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_state(AppState::GamePlaying) // TODO
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_board)
        .add_startup_system(setup_stats_boards)
        .add_event::<GameOverEvent>()
        .add_system_set(
            SystemSet::on_update(AppState::GamePlaying)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)) // TODO 加了FixedTimestep后无法通过State控制
                .with_system(manually_move_piece),
        )
        .add_system_set_to_stage(
            CoreStage::Update,  // TODO 无法改成PostUpdate
            SystemSet::on_update(AppState::GamePlaying)
            .with_system(check_collision)
            .with_system(check_full_line)
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(AppState::GamePlaying)
                .with_system(check_game_over)
                .with_system(rotate_piece)
                .with_system(auto_move_piece_down)
                .with_system(auto_generate_new_piece)
                .with_system(update_scoreboard)
                .with_system(update_linesboard)
        )
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_game_over_menu))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(debug_game_over_menu))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
