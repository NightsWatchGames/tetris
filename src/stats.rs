use bevy::prelude::*;

use crate::board::BLOCK_LENGTH;

// 计分板长宽
const STATS_BOARD_LENGTH: f32 = 250.0;
const STATS_BOARD_WIDTH: f32 = 50.0;

// 分数
#[derive(Resource)]
pub struct Score(pub u32);
#[derive(Component)]
pub struct Scoreboard;

// 消除行数
#[derive(Resource)]
pub struct Lines(pub u32);
#[derive(Component)]
pub struct Linesboard;

pub fn setup_stats_boards(mut commands: Commands, asset_server: Res<AssetServer>, windows: Res<Windows>) {
    // 通过窗口大小和棋盘大小计算stats位置
    let window = windows.primary();
    // gameboard左上角在窗口上的位置
    let gameboard_left_corner_pos = (window.width() / 2.0 - 5.0 * BLOCK_LENGTH, window.height() / 2.0 - 10.0 * BLOCK_LENGTH);
    info!("gameboard_left_corner_pos: {:?}", gameboard_left_corner_pos);
    // 分数
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 40.0,
                color: Color::rgb(1.0, 0.5, 0.5),
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(gameboard_left_corner_pos.1),
                left: Val::Px(gameboard_left_corner_pos.0 - STATS_BOARD_LENGTH),
                ..default()
            },
            ..default()
        }),
    ).insert(Scoreboard);

    // 行数
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Lines: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.5, 0.5, 1.0),
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 40.0,
                color: Color::rgb(1.0, 0.5, 0.5),
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(gameboard_left_corner_pos.1 + STATS_BOARD_WIDTH),
                left: Val::Px(gameboard_left_corner_pos.0 - STATS_BOARD_LENGTH),
                ..default()
            },
            ..default()
        }),
    ).insert(Linesboard);
}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}

pub fn update_linesboard(lines: Res<Lines>, mut query: Query<&mut Text, With<Linesboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = lines.0.to_string();
}

pub fn clear_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

pub fn clear_lines(mut lines: ResMut<Lines>) {
    lines.0 = 0;
}