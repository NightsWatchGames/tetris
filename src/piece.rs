use bevy::color::palettes;
use std::collections::{BTreeSet, VecDeque};

use crate::{board::*, common::GameAudios};
use bevy::prelude::*;
use rand::Rng;

const SHAPE_I: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [6, 0]];
const SHAPE_J: [[i32; 2]; 4] = [[3, 1], [3, 0], [4, 0], [5, 0]];
const SHAPE_L: [[i32; 2]; 4] = [[3, 0], [4, 0], [5, 0], [5, 1]];
const SHAPE_O: [[i32; 2]; 4] = [[4, 1], [4, 0], [5, 1], [5, 0]];
const SHAPE_S: [[i32; 2]; 4] = [[3, 0], [4, 0], [4, 1], [5, 1]];
const SHAPE_T: [[i32; 2]; 4] = [[3, 0], [4, 1], [4, 0], [5, 0]];
const SHAPE_Z: [[i32; 2]; 4] = [[3, 1], [4, 1], [4, 0], [5, 0]];

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

fn shift_block(mut block: Block, delta_x: Option<i32>, delta_y: Option<i32>) -> Block {
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

// 四格骨牌
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
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
        let color = Color::Srgba(match piece_type {
            PieceType::I => palettes::css::LIGHT_CYAN,
            PieceType::J => palettes::css::BLUE,
            PieceType::L => palettes::css::ORANGE,
            PieceType::O => palettes::css::YELLOW,
            PieceType::S => palettes::css::GREEN,
            PieceType::T => palettes::css::PURPLE,
            PieceType::Z => palettes::css::RED,
        });
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
    mut commands: Commands,
    game_audios: Res<GameAudios>,
    mut query: Query<(&mut Block, &mut Transform, &Movable), With<PieceType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut manually_move_timer: ResMut<ManuallyMoveTimer>,
    mut auto_move_timer: ResMut<AutoMovePieceDownTimer>,
    time: Res<Time>,
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

            spawn_drop_audio(&mut commands, &game_audios);
            already_down = true;
        }
        // 手动移动
        if manually_move_timer.0.finished() {
            if keyboard_input.pressed(KeyCode::ArrowLeft) && movable.can_left {
                block.x -= 1;
                spawn_drop_audio(&mut commands, &game_audios);
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::ArrowRight) && movable.can_right {
                block.x += 1;
                spawn_drop_audio(&mut commands, &game_audios);
                reset_manually_move_timer = true;
            } else if keyboard_input.pressed(KeyCode::ArrowDown)
                && movable.can_down
                && !already_down
            {
                block.y -= 1;
                spawn_drop_audio(&mut commands, &game_audios);
                reset_manually_move_timer = true;
            }
        }
        transform.translation = block.translation();
    }
    if reset_manually_move_timer {
        manually_move_timer.0.reset();
    }
}

fn spawn_drop_audio(commands: &mut Commands, game_audios: &Res<GameAudios>) {
    commands.spawn(AudioBundle {
        source: game_audios.drop.clone(),
        settings: PlaybackSettings::DESPAWN,
    });
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

pub fn rotate_piece(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_piece: Query<(&mut PieceType, &mut Block, &mut Transform)>,
    q_board: Query<&Block, Without<PieceType>>,
) {
    if keyboard_input.just_pressed(KeyCode::ArrowUp) {
        let piece_type = match q_piece.iter().next() {
            Some((piece_type, _, _)) => piece_type.clone(),
            None => {
                return;
            }
        };
        let sum_x = q_piece.iter().map(|(_, block, _)| block.x).sum::<i32>();
        let sum_y = q_piece.iter().map(|(_, block, _)| block.y).sum::<i32>();

        let original_blocks: Vec<Block> =
            q_piece.iter().map(|(_, block, _)| block.clone()).collect();
        // 通过矩阵变化实现旋转，可以理解为沿y=x对称后沿y=0对称，然后平移
        for (_, mut block, mut transform) in &mut q_piece {
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

        // 当出现碰撞时，尝试左右平移最多2格（也可采取旋转后一旦出现碰撞则恢复原样）
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                *block = shift_block(block.clone(), Some(-1), None);
                transform.translation = block.translation();
            }
        }
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                *block = shift_block(block.clone(), Some(-1), None);
                transform.translation = block.translation();
            }
        }
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                *block = shift_block(block.clone(), Some(3), None);
                transform.translation = block.translation();
            }
        }
        if whether_colliding(&q_piece, &q_board) {
            for (_, mut block, mut transform) in &mut q_piece {
                *block = shift_block(block.clone(), Some(3), None);
                transform.translation = block.translation();
            }
        }
        // 恢复旋转前样子
        if whether_colliding(&q_piece, &q_board) {
            let mut index = 0;
            for (_, mut block, mut transform) in &mut q_piece {
                *block = original_blocks[index];
                transform.translation = block.translation();
                index += 1;
            }
        }
    }
}

// 检测旋转过程中是否发送碰撞
pub fn whether_colliding(
    piece_query: &Query<(&mut PieceType, &mut Block, &mut Transform)>,
    board_query: &Query<&Block, Without<PieceType>>,
) -> bool {
    // 检查是否碰撞边界
    for (_, block, _) in piece_query {
        if block.x < 0 {
            // 碰撞左边界
            return true;
        }
        if block.x > 9 {
            // 碰撞右边界
            return true;
        }
        if block.y < 0 {
            // 碰撞下边界
            return true;
        }
    }

    // 检查是否碰撞面板方块
    for (_, block, _) in piece_query {
        for board_block in board_query {
            if board_block.y == block.y && block.x > 0 && board_block.x == block.x - 1 {
                // 防止0-1溢出
                // 左侧碰撞
                return true;
            }
            if board_block.y == block.y && board_block.x == block.x + 1 {
                // 右侧碰撞
                return true;
            }
            if board_block.x == block.x && block.y > 0 && board_block.y == block.y - 1 {
                // 下侧碰撞
                return true;
            }
        }
    }
    return false;
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
pub fn random_7_pieces() -> Vec<PieceConfig> {
    let mut rng = rand::thread_rng();
    let mut piece_type_set = BTreeSet::new();

    loop {
        match rng.gen_range(0..PieceType::PIECE_AMOUNT) {
            0 => {
                piece_type_set.insert(PieceType::I);
            }
            1 => {
                piece_type_set.insert(PieceType::J);
            }
            2 => {
                piece_type_set.insert(PieceType::L);
            }
            3 => {
                piece_type_set.insert(PieceType::O);
            }
            4 => {
                piece_type_set.insert(PieceType::S);
            }
            5 => {
                piece_type_set.insert(PieceType::T);
            }
            6 => {
                piece_type_set.insert(PieceType::Z);
            }
            _ => {
                panic!("Random value is unexpected");
            }
        }
        if piece_type_set.len() == PieceType::PIECE_AMOUNT as usize {
            break;
        }
    }
    // info!("random 7 pieces: {:?}", result);
    piece_type_set
        .iter()
        .map(|piece_type| {
            PieceConfig::new(
                *piece_type,
                shift_piece(piece_shape(*piece_type), None, Some(20)),
            )
        })
        .collect()
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
