use bevy::prelude::*;

// 正方形方块边长
pub const BLOCK_LENGTH: f32 = 30.0;

// game board 边界厚度
pub const BORDER_THICKNESS: f32 = 10.0;

// 方块
#[derive(Component, Clone, Debug)]
pub struct Block {
    pub x: u32,
    pub y: u32,
}

impl Block {
    pub fn new(x: u32, y: u32) -> Self {
        Block { x, y }
    }
    pub fn translation(&self) -> Vec3 {
        // 10*20个方块
        Vec3 { 
            x: (self.x as f32 - 5.0 + 0.5) * BLOCK_LENGTH, 
            y: (self.y as f32 - 10.0 + 0.5) * BLOCK_LENGTH, 
            z: 0.0 
        }
    }
}

pub fn setup_game_board(mut commands: Commands) {
    // 左侧边界
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3 { x: -5.0 * BLOCK_LENGTH - BORDER_THICKNESS / 2.0, y: 0.0, z: 0.0 },
            scale: Vec3 { x: BORDER_THICKNESS, y: 20.0 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS, z: 0.0 },
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
            translation: Vec3 { x: 5.0 * BLOCK_LENGTH + BORDER_THICKNESS / 2.0, y: 0.0, z: 0.0 },
            scale: Vec3 { x: BORDER_THICKNESS, y: 20.0 * BLOCK_LENGTH + 2.0 * BORDER_THICKNESS, z: 0.0 },
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
            translation: Vec3 { x: 0.0, y: 10.0 * BLOCK_LENGTH + BORDER_THICKNESS / 2.0, z: 0.0 },
            scale: Vec3 { x: 10.0 * BLOCK_LENGTH, y: BORDER_THICKNESS, z: 0.0 },
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
            translation: Vec3 { x: 0.0, y: -10.0 * BLOCK_LENGTH - BORDER_THICKNESS / 2.0, z: 0.0 },
            scale: Vec3 { x: 10.0 * BLOCK_LENGTH, y: BORDER_THICKNESS, z: 0.0 },
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.8, 0.8, 0.8),
            ..default()
        },
        ..default()
    });
}