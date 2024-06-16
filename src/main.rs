use std::time::Duration;

use bevy::{prelude::*, transform::TransformSystem};
// use bevy_inspector_egui::prelude::*;
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

fn main() {
    App::new()
        .insert_resource(Score(0))
        .insert_resource(Lines(0))
        .insert_resource(ClearColor(Color::BLACK))
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
        )))
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_game_board,
                setup_game_audios,
                setup_stats_boards,
                setup_piece_queue,
            ),
        )
        // Main Menu
        .add_systems(
            OnEnter(AppState::MainMenu),
            (
                setup_main_menu,
                clear_game_board,
                reset_score,
                reset_lines,
                clear_next_piece_board,
            ),
        )
        .add_systems(
            OnExit(AppState::MainMenu),
            despawn_screen::<OnMainMenuScreen>,
        )
        // Game Over Menu
        .add_systems(OnEnter(AppState::GameOver), setup_game_over_menu)
        .add_systems(
            OnExit(AppState::GameOver),
            (
                despawn_screen::<OnGameOverMenuScreen>,
                clear_game_board,
                reset_score,
                reset_lines,
                clear_next_piece_board,
            ),
        )
        // Game Playing
        .add_systems(
            PostUpdate,
            (
                check_collision,
                remove_piece_component,
                check_game_over.after(remove_piece_component),
                check_full_line
                    .after(remove_piece_component)
                    .before(TransformSystem::TransformPropagate),
            )
                .run_if(in_state(GameState::GamePlaying)),
        )
        .add_systems(
            Update,
            (
                rotate_piece,
                move_piece,
                auto_generate_new_piece,
                update_scoreboard,
                update_linesboard,
                update_next_piece_board,
                control_piece_visibility,
            )
                .run_if(in_state(GameState::GamePlaying)),
        )
        .add_systems(OnEnter(GameState::GamePaused), setup_game_paused_menu)
        // Game Paused
        .add_systems(
            OnExit(GameState::GamePaused),
            despawn_screen::<OnGamePausedMenuScreen>,
        )
        // Game Restarted
        .add_systems(
            OnEnter(GameState::GameRestarted),
            (clear_game_board, reset_score, reset_lines),
        )
        .add_systems(Update, play_game.run_if(in_state(GameState::GameRestarted)))
        // Common
        .add_systems(
            Update,
            pause_game
                .run_if(in_state(GameState::GamePlaying).or_else(in_state(GameState::GamePaused))),
        )
        .add_systems(
            Update,
            click_button.run_if(
                in_state(AppState::MainMenu)
                    .or_else(in_state(AppState::GameOver))
                    .or_else(in_state(GameState::GamePaused)),
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
