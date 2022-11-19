use crate::board::{Block, BLOCK_LENGTH};
use bevy::{prelude::*, transform};

// 四格骨牌
#[derive(Component)]
pub enum Piece {
    // ####
    I,

    // #
    // ###
    J,

    //   #
    // ###
    L,

    // ##
    // ##
    O,

    //  ##
    // ##
    S,

    //  #
    // ###
    T,

    // ##
    //  ##
    Z,
}

pub fn spawn_piece(mut commands: Commands) {
    let new_sprite_bundle = |block: &Block| SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.7, 0.7, 0.7),
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(BLOCK_LENGTH, BLOCK_LENGTH, BLOCK_LENGTH),
            translation: block.translation(),
            ..default()
        },
        ..default()
    };
    let block = Block::new(0, 0);
    commands.spawn(Piece::I).insert(new_sprite_bundle(&block)).insert(block);
    let block = Block::new(1, 0);
    commands.spawn(Piece::I).insert(new_sprite_bundle(&block)).insert(block);
    let block = Block::new(2, 0);
    commands.spawn(Piece::I).insert(new_sprite_bundle(&block)).insert(block);
    let block = Block::new(3, 0);
    commands.spawn(Piece::I).insert(new_sprite_bundle(&block)).insert(block);
}

// 移动四格骨牌
pub fn move_piece(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Block, &mut Transform), With<Piece>>) {
    
    // 找出边界
    let mut min_block_x = 9;
    let mut max_block_x = 0;
    for (block, _) in &query {
        if block.x < min_block_x {
            min_block_x = block.x;
        }
        if block.x > max_block_x {
            max_block_x = block.x;
        }
    }
    
    if keyboard_input.just_pressed(KeyCode::Left) {
        if min_block_x > 0 {
            for (mut block, mut transform) in &mut query {
                block.x -= 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        if max_block_x < 9 {
            for (mut block, mut transform) in &mut query {
                block.x += 1;
                transform.translation = block.translation();
            }
        }
    }
}

// 旋转四格骨牌
pub fn rotate_piece(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Block, &mut Transform), With<Piece>>) {

}