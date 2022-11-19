use crate::board::*;
use bevy::prelude::*;
use rand::Rng;

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

pub struct PieceConfig {
    init_blocks: (Block, Block, Block, Block),
    color: Color,
}

// 可移动方向
#[derive(Component)]
pub struct Movable {
    pub can_down: bool,
    pub can_left: bool,
    pub can_right: bool,
}

// 自动向下移动四格骨牌计时器
#[derive(Component, Deref, DerefMut)]
pub struct AutoMovePieceDownTimer(pub Timer);

// 手动移动四格骨牌
pub fn manually_move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Block, &mut Transform, &Movable), With<Piece>>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_left {
                block.x -= 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_right {
                block.x += 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_down {
                block.y -= 1;
                transform.translation = block.translation();
            }
        }
    }
}

// 自动向下移动四格骨牌
pub fn auto_move_piece_down(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut AutoMovePieceDownTimer,
            &mut Block,
            &mut Transform,
            &Movable,
        ),
        With<Piece>,
    >,
) {
    for (entity, mut timer, mut block, mut transform, movable) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if movable.can_down {
                block.y -= 1;
                transform.translation = block.translation();
            } else {
                // 移除piece组件
                info!("remove piece");
                commands.entity(entity).remove::<Piece>();
            }
        }
    }
}

// 检查碰撞
pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut Movable), With<Piece>>,
    board_query: Query<&Block, Without<Piece>>,
) {
    let mut can_down = true;
    let mut can_left = true;
    let mut can_right = true;

    // 检查是否碰撞边界
    for (block, _) in &mut piece_query {
        if block.x == 0 {
            // 碰撞左边界
            can_left = false;
        }
        if block.x == 9 {
            // 碰撞右边界
            can_right = false;
        }
        if block.y == 0 {
            // 碰撞下边界
            can_down = false;
        }
    }

    // 检查是否碰撞面板方块
    for (block, _) in &piece_query {
        for board_block in &board_query {
            if board_block.y == block.y && block.x > 0 && board_block.x == block.x - 1  {  // 防止0-1溢出
                // 左侧碰撞
                can_left = false;
            }
            if board_block.y == block.y && board_block.x == block.x + 1  {
                // 右侧碰撞
                can_right = false;
            }
            if board_block.x == block.x && block.y > 0 && board_block.y == block.y - 1  {
                // 下侧碰撞
                can_down = false;
            }
        }
    }

    // 更新Movable
    for (_, mut movable) in &mut piece_query {
        movable.can_left = can_left;
        movable.can_right = can_right;
        movable.can_down = can_down;
    }
}

// TODO 旋转四格骨牌
pub fn rotate_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Block, &mut Transform), With<Piece>>,
) {
}

pub fn auto_generate_new_piece(mut commands: Commands, query: Query<&Piece>) {
    if query.is_empty() {
        info!("generate piece");
        let piece_config = random_piece();
        // 生成新的四格骨牌
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
        let block = piece_config.init_blocks.0.clone();
        commands
            .spawn(Piece::I)
            .insert(new_sprite_bundle(&block))
            .insert(block)
            .insert(Movable {
                can_down: true,
                can_left: true,
                can_right: true,
            })
            .insert(AutoMovePieceDownTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )));
        let block = piece_config.init_blocks.1.clone();
        commands
            .spawn(Piece::I)
            .insert(new_sprite_bundle(&block))
            .insert(block)
            .insert(Movable {
                can_down: true,
                can_left: true,
                can_right: true,
            })
            .insert(AutoMovePieceDownTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )));
        let block = piece_config.init_blocks.2.clone();
        commands
            .spawn(Piece::I)
            .insert(new_sprite_bundle(&block))
            .insert(block)
            .insert(Movable {
                can_down: true,
                can_left: true,
                can_right: true,
            })
            .insert(AutoMovePieceDownTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )));
        let block = piece_config.init_blocks.3.clone();
        commands
            .spawn(Piece::I)
            .insert(new_sprite_bundle(&block))
            .insert(block)
            .insert(Movable {
                can_down: true,
                can_left: true,
                can_right: true,
            })
            .insert(AutoMovePieceDownTimer(Timer::from_seconds(
                1.0,
                TimerMode::Repeating,
            )));
    }
}

fn random_piece() -> PieceConfig {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..7) {
        0 => {
            // Piece::I
            PieceConfig {
                init_blocks: (Block::new(3, 19), Block::new(4, 19), Block::new(5, 19), Block::new(6, 19)),
                color: Color::BLACK,
            }
        },
        1 => {
            // Piece::J
            PieceConfig {
                init_blocks: (Block::new(4, 20), Block::new(4, 19), Block::new(5, 19), Block::new(6, 19)),
                color: Color::BLACK,
            }
        },
        2 => {
            // Piece::L
            PieceConfig {
                init_blocks: (Block::new(3, 19), Block::new(4, 19), Block::new(5, 19), Block::new(5, 20)),
                color: Color::BLACK,
            }
        },
        3 => {
            // Piece::O
            PieceConfig {
                init_blocks: (Block::new(4, 20), Block::new(4, 19), Block::new(5, 19), Block::new(5, 20)),
                color: Color::BLACK,
            }
        },
        4 => {
            // Piece::S
            PieceConfig {
                init_blocks: (Block::new(3, 19), Block::new(4, 19), Block::new(4, 20), Block::new(5, 20)),
                color: Color::BLACK,
            }
        },
        5 => {
            // Piece::T
            PieceConfig {
                init_blocks: (Block::new(3, 19), Block::new(4, 20), Block::new(4, 19), Block::new(5, 19)),
                color: Color::BLACK,
            }
        },
        6 => {
            // Piece::Z
            PieceConfig {
                init_blocks: (Block::new(4, 20), Block::new(5, 20), Block::new(5, 19), Block::new(6, 19)),
                color: Color::BLACK,
            }
        },
        _ => { panic!("No matched piece") },
    }
}