use std::collections::HashMap;
use std::collections::HashSet;

use bevy::prelude::*;

use crate::piece::*;
use crate::stats::*;
use crate::common::*;

// 正方形方块边长
pub const BLOCK_LENGTH: f32 = 30.0;

// game board 边界厚度
pub const BORDER_THICKNESS: f32 = 10.0;

// 方块
#[derive(Component, Clone, Copy, Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

#[derive(Default)]
pub struct GameOverEvent;

impl Block {
    pub fn new(x: u32, y: u32) -> Self {
        Block { x, y }
    }
    pub fn translation(&self) -> Vec3 {
        // 10*20个方块
        Vec3 {
            x: (self.x as f32 - 5.0 + 0.5) * BLOCK_LENGTH,
            y: (self.y as f32 - 10.0 + 0.5) * BLOCK_LENGTH,
            z: 0.0,
        }
    }
    pub fn set(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

pub fn setup_game_board(mut commands: Commands) {
    // 左侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: -5.0 * BLOCK_LENGTH - BORDER_THICKNESS / 2.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: 20.0 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.8, 0.8, 0.8),
            ..default()
        },
        ..default()
    });
    // 右侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 5.0 * BLOCK_LENGTH + BORDER_THICKNESS / 2.0,
                y: 0.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: BORDER_THICKNESS,
                y: 20.0 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.8, 0.8, 0.8),
            ..default()
        },
        ..default()
    });
    // 上侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: 10.0 * BLOCK_LENGTH + BORDER_THICKNESS / 2.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 10.0 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.8, 0.8, 0.8),
            ..default()
        },
        ..default()
    });
    // 下侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 {
                x: 0.0,
                y: -10.0 * BLOCK_LENGTH - BORDER_THICKNESS / 2.0,
                z: 0.0,
            },
            scale: Vec3 {
                x: 10.0 * BLOCK_LENGTH,
                y: BORDER_THICKNESS,
                z: 0.0,
            },
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.8, 0.8, 0.8),
            ..default()
        },
        ..default()
    });
}

// 检查是否有成功的行
pub fn check_full_line(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut lines: ResMut<Lines>,
    mut query: Query<(Entity, &mut Block, &mut Transform), Without<Piece>>,
) {
    let mut y_to_x_set_map: HashMap<u32, HashSet<u32>> = HashMap::new();
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
        if x_set.len() == 10 {
            full_lines.push(y);
        }
    }
    if full_lines.len() > 0 {
        dbg!(full_lines.len());
    }
    // 行数增加
    lines.0 += full_lines.len() as u32;
    // 分数增加
    score.0 += match full_lines.len() {
        0 => { 0 },
        1 => { 100 },
        2 => { 200 },
        3 => { 400 },
        4 => { 800 },
        _ => { panic!("No matched score") },
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
    // 其余行向下移
    for (entity, mut block, mut transform) in &mut query {
        if !despawn_entities.contains(&entity) {
            block.y -= full_lines.len() as u32;
            transform.translation = block.translation();
        }
    }
}


// 检查是否游戏结束
pub fn check_game_over(
    mut state: ResMut<State<AppState>>,
    query: Query<&Block, Without<Piece>>, 
    mut game_over_events: EventWriter<GameOverEvent>,) {
    let mut max_block_y = 0;
    for block in &query {
        if block.y > max_block_y {
            max_block_y = block.y;
        }
    }
    if max_block_y >= 19 {
        game_over_events.send(GameOverEvent::default());
        state.set(AppState::GameOver).unwrap();
    }
}