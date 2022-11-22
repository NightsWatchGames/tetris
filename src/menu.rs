use bevy::prelude::*;

use crate::{board::*, common::AppState};

// TODO 菜单
pub fn setup_game_over_menu(mut commands: Commands) {
    println!("Show game over menu");
}

pub fn debug_game_over_menu(mut commands: Commands, state: Res<State<AppState>>) {
    dbg!(state);
}