use std::collections::VecDeque;

use crate::{board::*, common::GameAudios};
use bevy::prelude::*;
use rand::Rng;

pub const SHAPE_I: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [6, 0]];
pub const SHAPE_J: [[i32; 2]; 4] = [[3, 1], [3, 0], [4, 0], [5, 0]];
pub const SHAPE_L: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [5, 1]];
pub const SHAPE_O: [[i32; 2]; 4] = [[4, 1], [4, 0], [5, 1], [5, 0]];
pub const SHAPE_S: [[i32; 2]; 4] = [[3, 0], [4, 0], [4, 1], [5, 1]];
pub const SHAPE_T: [[i32; 2]; 4] = [[3, 0], [4, 1], [4, 0], [5, 0]];
pub const SHAPE_Z: [[i32; 2]; 4] = [[3, 1], [4, 1], [4, 0], [5, 0]];

pub fn piece_shape(piece_type: PieceType) -> [Block; 4] {
    match piece_type {
        PieceType::I => SHAPE_I.map(|pos| pos.into()),
        PieceType::J => SHAPE_J.map(|pos| pos.into()),
        PieceType::L => SHAPE_L.map(|pos| pos.into()),
        PieceType::O => SHAPE_O.map(|pos| pos.into()),
        PieceType::S => SHAPE_S.map(|pos| pos.into()),
        PieceType::T => SHAPE_T.map(|pos| pos.into()),
        PieceType::Z => SHAPE_Z.map(|pos| pos.into()),
    }
}

// 平移骨牌
pub fn shift_piece(
    mut blocks: [Block; 4],
    delta_x: Option<i32>,
    delta_y: Option<i32>,
) -> [Block; 4] {
    match delta_x {
        Some(delta) => {
            blocks[0].x += delta;
            blocks[1].x += delta;
            blocks[2].x += delta;
            blocks[3].x += delta;
        }
        None => {}
    }
    match delta_y {
        Some(delta) => {
            blocks[0].y += delta;
            blocks[1].y += delta;
            blocks[2].y += delta;
            blocks[3].y += delta;
        }
        None => {}
    }
    blocks
}

pub fn shift_block(mut block: Block, delta_x: Option<i32>, delta_y: Option<i32>) -> Block {
    match delta_x {
        Some(delta) => {
            block.x += delta;
        }
        None => {}
    }
    match delta_y {
        Some(delta) => {
            block.y += delta;
        }
        None => {}
    }
    block
}

lazy_static::lazy_static!(
    static ref GENERATED_PIECES: Vec<PieceConfig> = vec![
        PieceConfig::new(
            PieceType::I,
            shift_piece(piece_shape(PieceType::I), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::J,
            shift_piece(piece_shape(PieceType::J), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::L,
            shift_piece(piece_shape(PieceType::L), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::O,
            shift_piece(piece_shape(PieceType::O), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::S,
            shift_piece(piece_shape(PieceType::S), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::T,
            shift_piece(piece_shape(PieceType::T), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::Z,
            shift_piece(piece_shape(PieceType::Z), None, Some(20))
        ),
    ];
);

// 四格骨牌
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
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

impl PieceType {
    pub const PIECE_AMOUNT: u32 = 7;
}

#[derive(Debug, Clone)]
pub struct PieceConfig {
    pub piece_type: PieceType,
    pub blocks: [Block; 4],
    pub color: Color,
}

impl PieceConfig {
    pub fn new(piece_type: PieceType, blocks: [Block; 4]) -> Self {
        let color = match piece_type {
            PieceType::I => Color::CYAN,
            PieceType::J => Color::BLUE,
            PieceType::L => Color::ORANGE,
            PieceType::O => Color::YELLOW,
            PieceType::S => Color::GREEN,
            PieceType::T => Color::PURPLE,
            PieceType::Z => Color::RED,
        };
        PieceConfig {
            piece_type,
            blocks,
            color,
        }
    }
}

// 可移动方向
#[derive(Component)]
pub struct Movable {
    pub can_down: bool,
    pub can_left: bool,
    pub can_right: bool,
}

// 自动向下移动四格骨牌计时器
#[derive(Debug, Resource)]
pub struct AutoMovePieceDownTimer(pub Timer);

// 待生成的骨牌队列
#[derive(Debug, Resource)]
pub struct PieceQueue(pub VecDeque<PieceConfig>);

// 控制手动移动频率
#[derive(Debug, Resource)]
pub struct ManuallyMoveTimer(pub Timer);

pub fn setup_piece_queue(mut commands: Commands) {
    let mut piece_queue = PieceQueue(VecDeque::new());
    piece_queue.0.extend(random_7_pieces());
    commands.insert_resource(piece_queue);
}

// 自动和手动移动四格骨牌
pub fn move_piece(
    mut query: Query<(&mut Block, &mut Transform, &Movable), With<PieceType>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut manually_move_timer: ResMut<ManuallyMoveTimer>,
    mut auto_move_timer: ResMut<AutoMovePieceDownTimer>,
    time: Res<Time>,
    audio: Res<Audio>,
    game_audios: Res<GameAudios>,
) {
    manually_move_timer.0.tick(time.delta());
    auto_move_timer.0.tick(time.delta());
    let mut reset_manually_move_timer = false;
    for (mut block, mut transform, movable) in &mut query {
        // 防止一帧中向下移动两行
        let mut already_down = false;
        // 自动下移
        if auto_move_timer.0.just_finished() && movable.can_down {
            block.y -= 1;
            audio.play(game_audios.drop.clone());
            already_down = true;
        }
        // 手动移动
        if manually_move_timer.0.finished() {
            if keyboard_input.pressed(KeyCode::Left) && movable.can_left {
                block.x -= 1;
                audio.play(game_audios.drop.clone());
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::Right) && movable.can_right {
                block.x += 1;
                audio.play(game_audios.drop.clone());
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::Down) && movable.can_down && !already_down {
                block.y -= 1;
                audio.play(game_audios.drop.clone());
                reset_manually_move_timer = true;
            }
        }
        transform.translation = block.translation();
    }
    if reset_manually_move_timer {
        manually_move_timer.0.reset();
    }
}

// 检查碰撞
pub fn check_collision(
    mut piece_query: Query<(&mut Block, &mut Movable), With<PieceType>>,
    board_query: Query<&Block, Without<PieceType>>,
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
            if board_block.y == block.y && block.x > 0 && board_block.x == block.x - 1 {
                // 防止0-1溢出
                // 左侧碰撞
                can_left = false;
            }
            if board_block.y == block.y && board_block.x == block.x + 1 {
                // 右侧碰撞
                can_right = false;
            }
            if board_block.x == block.x && block.y > 0 && board_block.y == block.y - 1 {
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

// TODO 旋转不能产生碰撞（进入物体）
pub fn rotate_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PieceType, &mut Block, &mut Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        let piece_type = match query.iter().next() {
            Some((piece_type, _, _)) => piece_type.clone(),
            None => {
                return;
            }
        };
        let sum_x = query.iter().map(|(_, block, _)| block.x).sum::<i32>();
        let sum_y = query.iter().map(|(_, block, _)| block.y).sum::<i32>();
        // 通过矩阵变化实现旋转，可以理解为沿y=x对称后沿y=0对称，然后平移
        for (_, mut block, mut transform) in &mut query {
            *block = match piece_type {
                // 微调平移量，使其更自然
                PieceType::O | PieceType::L | PieceType::J => shift_block(
                    [block.y, -block.x].into(),
                    Some(sum_x / 4 - sum_y / 4),
                    Some(sum_x / 4 + sum_y / 4 + 1),
                ),
                _ => shift_block(
                    [block.y, -block.x].into(),
                    Some(sum_x / 4 - sum_y / 4),
                    Some(sum_x / 4 + sum_y / 4),
                ),
            };
            transform.translation = block.translation();
        }
    }
}

pub fn control_piece_visibility(mut q_piece: Query<(&mut Visibility, &Block), With<PieceType>>) {
    for (mut visibility, block) in &mut q_piece {
        if block.y > 19 {
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
    }
}

// 自动生成新的四格骨牌
pub fn auto_generate_new_piece(
    mut commands: Commands,
    query: Query<&PieceType>,
    mut piece_queue: ResMut<PieceQueue>,
) {
    if piece_queue.0.len() < PieceType::PIECE_AMOUNT as usize {
        piece_queue.0.extend(random_7_pieces());
    }
    if query.is_empty() {
        let piece_config = piece_queue.0.pop_front().unwrap();
        // 生成新的四格骨牌
        let color = piece_config.color;
        let visibility = Visibility::Hidden;
        let piece_type = piece_config.piece_type.clone();

        for block in piece_config.blocks.iter() {
            commands
                .spawn(piece_type)
                .insert(new_block_sprite(&block, color, visibility))
                .insert(*block)
                .insert(Movable {
                    can_down: true,
                    can_left: true,
                    can_right: true,
                });
        }
    }
}

// bag7算法实现随机：每次填充7个随机排序的骨牌
fn random_7_pieces() -> Vec<PieceConfig> {
    let mut result: Vec<PieceConfig> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut rand_ints: Vec<u32> = Vec::new();

    loop {
        let mut select = |rand_int: u32| {
            if !rand_ints.contains(&rand_int) {
                rand_ints.push(rand_int);
                result.push(GENERATED_PIECES.get(rand_int as usize).unwrap().clone());
            }
        };
        match rng.gen_range(0..PieceType::PIECE_AMOUNT) {
            0 => {
                select(0);
            }
            1 => {
                select(1);
            }
            2 => {
                select(2);
            }
            3 => {
                select(3);
            }
            4 => {
                select(4);
            }
            5 => {
                select(5);
            }
            6 => {
                select(6);
            }
            _ => {
                panic!("Random value is unexpected");
            }
        }
        if result.len() == PieceType::PIECE_AMOUNT as usize {
            break;
        }
    }
    // info!("random 7 pieces: {:?}", result);
    result
}

pub fn new_block_sprite(block: &Block, color: Color, visibility: Visibility) -> SpriteBundle {
    SpriteBundle {
        sprite: Sprite { color, ..default() },
        transform: Transform {
            scale: Vec3::new(
                BLOCK_STICKER_LENGTH,
                BLOCK_STICKER_LENGTH,
                BLOCK_STICKER_LENGTH,
            ),
            translation: block.translation(),
            ..default()
        },
        visibility,
        ..default()
    }
}
