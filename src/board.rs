use std::collections::HashMap;
use std::collections::HashSet;

use bevy::prelude::*;

use crate::common::*;
use crate::piece::*;
use crate::stats::*;

// game board宽高
pub const COL_COUNT: u8 = 10;
pub const ROW_COUNT: u8 = 20;
// 正方形方块边长
pub const BLOCK_LENGTH: f32 = 30.0;
// TODO 贴纸圆角
// 正方形方块贴纸边长
pub const BLOCK_STICKER_LENGTH: f32 = 28.0;

// game board 边界厚度
pub const BORDER_THICKNESS: f32 = 10.0;
pub const BORDER_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// 方块
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
}

#[derive(Default)]
pub struct GameOverEvent;

impl Block {
    pub fn translation(&self) -> Vec3 {
        // 方块xy原点为左下角
        // 方块x范围0-9，方块y范围0-19
        // 10*20个方块
        Vec3 {
            x: (self.x as f32 - (COL_COUNT as f32 / 2.0) + 0.5) * BLOCK_LENGTH,
            y: (self.y as f32 - (ROW_COUNT as f32 / 2.0) + 0.5) * BLOCK_LENGTH,
            z: 0.0,
        }
    }
}

impl From<[i32; 2]> for Block {
    fn from([x, y]: [i32; 2]) -> Self {
        Block { x, y }
    }
}

pub fn setup_game_board(mut commands: Commands) {
    // 三维坐标原点在board中央
    // 左侧边界
    let half_col_count = COL_COUNT as f32 / 2.0;
    let half_raw_count = ROW_COUNT as f32 / 2.0;
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: -half_col_count * BLOCK_LENGTH - BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: ROW_COUNT as f32 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // 右侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: half_col_count * BLOCK_LENGTH + BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: ROW_COUNT as f32 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // 上侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                y: half_raw_count * BLOCK_LENGTH + BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: COL_COUNT as f32 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
    // 下侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                y: -half_raw_count * BLOCK_LENGTH - BORDER_THICKNESS / 2.0,
                ..default()
            },
            scale: Vec3 {
                x: COL_COUNT as f32 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: BORDER_COLOR,
            ..default()
        },
        ..default()
    });
}

// 当piece移到底部后，移除piece组件
pub fn remove_piece_component(
    mut commands: Commands,
    query: Query<(Entity, &Movable), With<PieceType>>,
) {
    for (entity, movable) in &query {
        if !movable.can_down {
            commands.entity(entity).remove::<PieceType>();
        }
    }
}

// 检查是否有成功的行
pub fn check_full_line(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut lines: ResMut<Lines>,
    mut query: Query<(Entity, &mut Block, &mut Transform), Without<PieceType>>,
    audio: Res<Audio>,
    game_audios: Res<GameAudios>,
) {
    let mut y_to_x_set_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for (_, block, _) in &query {
        if y_to_x_set_map.contains_key(&block.y) {
            let x_set = y_to_x_set_map.get_mut(&block.y).unwrap();
            x_set.insert(block.x);
        } else {
            let mut x_set = HashSet::new();
            x_set.insert(block.x);
            y_to_x_set_map.insert(block.y, x_set);
        }
    }
    let mut full_lines = Vec::new();
    for (y, x_set) in y_to_x_set_map.iter() {
        if x_set.len() == COL_COUNT as usize {
            full_lines.push(y);
        }
    }
    if full_lines.len() > 0 {
        dbg!(full_lines.len());
        audio.play(game_audios.line_clear.clone());
    }
    // 行数增加
    lines.0 += full_lines.len() as u32;
    // 分数增加
    score.0 += match full_lines.len() {
        0 => 0,
        1 => 100,
        2 => 200,
        3 => 400,
        4 => 800,
        _ => {
            panic!("No matched score")
        }
    };

    // 消除行
    let mut despawn_entities = Vec::new();
    for line_no in full_lines.iter() {
        let line_no = line_no.clone().to_owned();
        for (entity, block, _) in &mut query {
            if block.y == line_no {
                despawn_entities.push(entity);
                commands.entity(entity).despawn();
            }
        }
    }
    // 消除行的上面block整体向下移
    full_lines.sort();
    full_lines.reverse();
    for line_no in full_lines.iter() {
        for (entity, mut block, mut transform) in &mut query {
            if !despawn_entities.contains(&entity) && block.y > line_no.clone().to_owned() {
                info!("down block: {:?}, line_no: {}", block, line_no);
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
}

// 检查是否游戏结束
pub fn check_game_over(
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Block, Without<PieceType>>,
    mut game_over_events: EventWriter<GameOverEvent>,
    audio: Res<Audio>,
    game_audios: Res<GameAudios>,
) {
    let mut max_block_y = 0;
    for block in &query {
        if block.y > max_block_y {
            max_block_y = block.y;
        }
    }
    // info!("max_block_y: {}", max_block_y);
    if max_block_y >= 19 {
        audio.play(game_audios.gameover.clone());
        game_over_events.send(GameOverEvent::default());
        app_state.set(AppState::GameOver);
        game_state.set(GameState::GameQuitted);
    }
}

pub fn clear_game_board(mut commands: Commands, query: Query<Entity, With<Block>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
