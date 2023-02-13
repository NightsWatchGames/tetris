use bevy::prelude::*;

use crate::{
    board::{Block, BLOCK_LENGTH, BLOCK_STICKER_LENGTH},
    new_block_sprite,
    piece::{Piece, Piece4Blocks, PieceConfig, PieceQueue},
};

// 计分板长宽
const STATS_BOARD_LENGTH: f32 = 220.0;
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

// 展示下一个骨牌
#[derive(Debug, Resource)]
pub struct NextPieceType(pub Option<Piece>);

#[derive(Debug, Component)]
pub struct NextPiece;

pub fn setup_stats_boards(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: Query<&Window>,
) {
    // 通过窗口大小和棋盘大小计算stats位置
    let window = windows.single();
    // gameboard左上角在窗口上的位置
    let gameboard_left_corner_pos = (
        window.width() / 2.0 - 5.0 * BLOCK_LENGTH,
        window.height() / 2.0 - 10.0 * BLOCK_LENGTH,
    );
    info!("gameboard_left_corner_pos: {:?}", gameboard_left_corner_pos);
    // 分数
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                ),
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
        )
        .insert(Scoreboard);

    // 行数
    commands
        .spawn(
            TextBundle::from_sections([
                TextSection::new(
                    "Lines: ",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.5, 0.5, 1.0),
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(1.0, 0.5, 0.5),
                    },
                ),
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
        )
        .insert(Linesboard);
}

pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<Scoreboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = score.0.to_string();
}

pub fn update_linesboard(lines: Res<Lines>, mut query: Query<&mut Text, With<Linesboard>>) {
    let mut text = query.single_mut();
    text.sections[1].value = lines.0.to_string();
}

pub fn reset_score(mut score: ResMut<Score>) {
    score.0 = 0;
}

pub fn reset_lines(mut lines: ResMut<Lines>) {
    lines.0 = 0;
}

pub fn update_next_piece_board(
    mut commands: Commands,
    piece_queue: Res<PieceQueue>,
    mut next_piece_type: ResMut<NextPieceType>,
    query: Query<Entity, With<NextPiece>>,
) {
    if next_piece_type.0.is_none()
        || piece_queue.0.front().unwrap().piece != next_piece_type.0.unwrap()
    {
        next_piece_type.0 = Some(piece_queue.0.front().unwrap().piece);
        // 销毁原board
        for entity in &query {
            commands.entity(entity).despawn();
        }
        let piece = piece_queue.0.front().unwrap().piece;
        let color = piece_queue.0.front().unwrap().color;
        match piece {
            Piece::I(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 18),
                    Block::new(12, 18),
                    Block::new(13, 18),
                    Block::new(14, 18),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
            Piece::J(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 17),
                    Block::new(12, 17),
                    Block::new(13, 17),
                    Block::new(11, 18),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
            Piece::L(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 17),
                    Block::new(12, 17),
                    Block::new(13, 17),
                    Block::new(13, 18),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
            Piece::O(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 17),
                    Block::new(12, 17),
                    Block::new(11, 18),
                    Block::new(12, 18),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
            Piece::S(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 17),
                    Block::new(12, 17),
                    Block::new(12, 18),
                    Block::new(13, 18),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
            Piece::T(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 17),
                    Block::new(12, 17),
                    Block::new(13, 17),
                    Block::new(12, 18),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
            Piece::Z(_) => {
                let piece4blocks = Piece4Blocks(
                    Block::new(11, 18),
                    Block::new(12, 18),
                    Block::new(12, 17),
                    Block::new(13, 17),
                );
                spawn_next_piece_board(&mut commands, piece4blocks, color);
            }
        }
    }
}

pub fn spawn_next_piece_board(commands: &mut Commands, piece4blocks: Piece4Blocks, color: Color) {
    let visibility = Visibility::Visible;
    commands
        .spawn(new_block_sprite(&piece4blocks.0, color, visibility))
        .insert(NextPiece);
    commands
        .spawn(new_block_sprite(&piece4blocks.1, color, visibility))
        .insert(NextPiece);
    commands
        .spawn(new_block_sprite(&piece4blocks.2, color, visibility))
        .insert(NextPiece);
    commands
        .spawn(new_block_sprite(&piece4blocks.3, color, visibility))
        .insert(NextPiece);
}

pub fn clear_next_piece_board(mut commands: Commands, query: Query<Entity, With<NextPiece>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
