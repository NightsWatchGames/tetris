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
        .add_state(AppState::MainMenu)
        .add_state(GameState::GameQuitted)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_board)
        .add_startup_system(setup_stats_boards)
        .add_startup_system(setup_game_audios)
        .add_event::<GameOverEvent>()
        // Main Menu
        .add_system_set(
            SystemSet::on_enter(AppState::MainMenu)
                .with_system(setup_main_menu)
                .with_system(clear_board)
                .with_system(clear_score)
                .with_system(clear_lines)
        )
        .add_system_set(SystemSet::on_update(AppState::MainMenu).with_system(click_button))
        .add_system_set(
            SystemSet::on_exit(AppState::MainMenu).with_system(despawn_screen::<OnMainMenuScreen>),
        )
        // Game Over Menu
        .add_system_set(SystemSet::on_enter(AppState::GameOver).with_system(setup_game_over_menu))
        .add_system_set(SystemSet::on_update(AppState::GameOver).with_system(click_button))
        .add_system_set(
            SystemSet::on_exit(AppState::GameOver)
                .with_system(despawn_screen::<OnGameOverMenuScreen>)
                .with_system(clear_board)
                .with_system(clear_score)
                .with_system(clear_lines)
        )
        // Game Playing
        .add_system_set(
            SystemSet::on_update(GameState::GamePlaying)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64)) // TODO 加了FixedTimestep后无法通过State控制
                .with_system(manually_move_piece),
        )
        .add_system_set_to_stage(
            CoreStage::Update, // TODO 无法改成PostUpdate
            SystemSet::on_update(GameState::GamePlaying)
                .with_system(remove_piece)
                .with_system(check_collision)
                .with_system(check_game_over.after(remove_piece))
                .with_system(check_full_line.after(remove_piece)),
        )
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(GameState::GamePlaying)
                .with_system(rotate_piece)
                .with_system(auto_move_piece_down)
                .with_system(auto_generate_new_piece)
                .with_system(update_scoreboard)
                .with_system(update_linesboard)
                .with_system(pause_game),
        )
        // Game Paused
        .add_system_set(
            SystemSet::on_enter(GameState::GamePaused).with_system(setup_game_paused_menu),
        )
        .add_system_set(SystemSet::on_update(GameState::GamePaused).with_system(click_button))
        .add_system_set(
            SystemSet::on_exit(GameState::GamePaused)
                .with_system(despawn_screen::<OnGamePausedMenuScreen>),
        )
        // Game Restarted
        .add_system_set(
            SystemSet::on_enter(GameState::GameRestarted)
                .with_system(clear_board)
                .with_system(clear_score)
                .with_system(clear_lines),
        )
        .add_system_set(SystemSet::on_update(GameState::GameRestarted).with_system(play_game))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn debug_state(state: Res<State<AppState>>) {
    dbg!(state);
}