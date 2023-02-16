use std::time::Duration;

use bevy::prelude::*;
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

const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .insert_resource(Score(0))
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
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_startup_system(setup_camera)
        .add_startup_system(setup_game_board)
        .add_startup_system(setup_stats_boards)
        .add_startup_system(setup_game_audios)
        .add_startup_system(setup_piece_queue)
        // Main Menu
        .add_systems_to_schedule(
            OnEnter(AppState::MainMenu),
            (
                setup_main_menu,
                clear_game_board,
                reset_score,
                reset_lines,
                clear_next_piece_board,
            ),
        )
        .add_system(click_button.in_set(OnUpdate(AppState::MainMenu)))
        .add_system_to_schedule(
            OnExit(AppState::MainMenu),
            despawn_screen::<OnMainMenuScreen>,
        )
        // Game Over Menu
        .add_system_to_schedule(OnEnter(AppState::GameOver), setup_game_over_menu)
        .add_system(click_button.in_set(OnUpdate(AppState::GameOver)))
        .add_systems_to_schedule(
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
        // TODO 待 https://github.com/bevyengine/bevy/issues/7659 支持后可以简化代码
        .add_system(
            check_collision
                .in_base_set(CoreSet::PostUpdate)
                .run_if(state_equals(GameState::GamePlaying)),
        )
        .add_system(
            remove_piece_component
                .in_base_set(CoreSet::PostUpdate)
                .run_if(state_equals(GameState::GamePlaying)),
        )
        .add_system(
            check_game_over
                .after(remove_piece_component)
                .in_base_set(CoreSet::PostUpdate)
                .run_if(state_equals(GameState::GamePlaying)),
        )
        .add_system(
            check_full_line
                .after(remove_piece_component)
                .in_base_set(CoreSet::PostUpdate)
                .run_if(state_equals(GameState::GamePlaying)),
        )
        .add_systems(
            (
                rotate_piece,
                move_piece,
                auto_generate_new_piece,
                update_scoreboard,
                update_linesboard,
                pause_game,
                update_next_piece_board,
                control_piece_visibility,
            )
                .in_set(OnUpdate(GameState::GamePlaying)),
        )
        // Game Paused
        .add_system_to_schedule(OnEnter(GameState::GamePaused), setup_game_paused_menu)
        .add_system(click_button.in_set(OnUpdate(GameState::GamePaused)))
        .add_system_to_schedule(
            OnExit(GameState::GamePaused),
            despawn_screen::<OnGamePausedMenuScreen>,
        )
        // Game Restarted
        .add_systems_to_schedule(
            OnEnter(GameState::GameRestarted),
            (clear_game_board, reset_score, reset_lines),
        )
        .add_system(play_game.in_set(OnUpdate(GameState::GameRestarted)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
