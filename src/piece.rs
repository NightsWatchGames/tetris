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
        PieceType::I(_) => SHAPE_I.map(|pos| pos.into()),
        PieceType::J(_) => SHAPE_J.map(|pos| pos.into()),
        PieceType::L(_) => SHAPE_L.map(|pos| pos.into()),
        PieceType::O(_) => SHAPE_O.map(|pos| pos.into()),
        PieceType::S(_) => SHAPE_S.map(|pos| pos.into()),
        PieceType::T(_) => SHAPE_T.map(|pos| pos.into()),
        PieceType::Z(_) => SHAPE_Z.map(|pos| pos.into()),
    }
}

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

lazy_static::lazy_static!(
    static ref ALL_PIECES: Vec<PieceConfig> = vec![
        PieceConfig::new(
            PieceType::I(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::I(RotationAngle::Angle0)), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::J(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::J(RotationAngle::Angle0)), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::L(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::L(RotationAngle::Angle0)), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::O(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::O(RotationAngle::Angle0)), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::S(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::S(RotationAngle::Angle0)), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::T(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::T(RotationAngle::Angle0)), None, Some(20))
        ),
        PieceConfig::new(
            PieceType::Z(RotationAngle::Angle0),
            shift_piece(piece_shape(PieceType::Z(RotationAngle::Angle0)), None, Some(20))
        ),
    ];
);

// 四格骨牌
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    // ####
    I(RotationAngle),

    // #
    // ###
    J(RotationAngle),

    //   #
    // ###
    L(RotationAngle),

    // ##
    // ##
    O(RotationAngle),

    //  ##
    // ##
    S(RotationAngle),

    //  #
    // ###
    T(RotationAngle),

    // ##
    //  ##
    Z(RotationAngle),
}

impl PieceType {
    pub const PIECE_AMOUNT: u32 = 7;
}

// 旋转角度
// TODO 参考 https://github.com/kunieone/tetris_rs 重构旋转部分
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationAngle {
    Angle0,
    Angle90,
    Angle180,
    Angle270,
}

#[derive(Debug, Clone)]
pub struct PieceConfig {
    pub piece: PieceType,
    pub blocks: [Block; 4],
    pub color: Color,
}

impl PieceConfig {
    pub fn new(piece: PieceType, blocks: [Block; 4]) -> Self {
        let color = match piece {
            PieceType::I(_) => Color::CYAN,
            PieceType::J(_) => Color::BLUE,
            PieceType::L(_) => Color::ORANGE,
            PieceType::O(_) => Color::YELLOW,
            PieceType::S(_) => Color::GREEN,
            PieceType::T(_) => Color::PURPLE,
            PieceType::Z(_) => Color::RED,
        };
        PieceConfig {
            piece,
            blocks,
            color,
        }
    }
}

pub fn blocks_min_x(blocks: &[Block; 4]) -> i32 {
    blocks[0]
        .x
        .min(blocks[1].x)
        .min(blocks[2].x)
        .min(blocks[3].x)
}
pub fn blocks_min_y(blocks: &[Block; 4]) -> i32 {
    blocks[0]
        .y
        .min(blocks[1].y)
        .min(blocks[2].y)
        .min(blocks[3].y)
}
pub fn blocks_max_x(blocks: &[Block; 4]) -> i32 {
    blocks[0]
        .x
        .max(blocks[1].x)
        .max(blocks[2].x)
        .max(blocks[3].x)
}
pub fn blocks_max_y(blocks: &[Block; 4]) -> i32 {
    blocks[0]
        .y
        .max(blocks[1].y)
        .max(blocks[2].y)
        .max(blocks[3].y)
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

// 手动移动四格骨牌
pub fn manually_move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Block, &mut Transform, &Movable), With<PieceType>>,
    mut timer: ResMut<ManuallyMoveTimer>,
    audio: Res<Audio>,
    game_audios: Res<GameAudios>,
    time: Res<Time>,
) {
    timer.0.tick(time.delta());
    if timer.0.finished() {
        if keyboard_input.pressed(KeyCode::Left) {
            for (_, mut block, mut transform, movable) in &mut query {
                if movable.can_left {
                    block.x -= 1;
                    transform.translation = block.translation();
                    audio.play(game_audios.drop.clone());
                    timer.0.reset();
                }
            }
        } else if keyboard_input.pressed(KeyCode::Right) {
            for (_, mut block, mut transform, movable) in &mut query {
                if movable.can_right {
                    block.x += 1;
                    transform.translation = block.translation();
                    audio.play(game_audios.drop.clone());
                    timer.0.reset();
                }
            }
        } else if keyboard_input.pressed(KeyCode::Down) {
            for (entity, mut block, mut transform, movable) in &mut query {
                if movable.can_down {
                    block.y -= 1;
                    transform.translation = block.translation();
                    audio.play(game_audios.drop.clone());
                    timer.0.reset();
                }
            }
        }
    }
}

// 自动向下移动四格骨牌
pub fn auto_move_piece_down(
    time: Res<Time>,
    mut query: Query<
        (
            &mut AutoMovePieceDownTimer,
            &mut Block,
            &mut Transform,
            &Movable,
        ),
        With<PieceType>,
    >,
    audio: Res<Audio>,
    game_audios: Res<GameAudios>,
) {
    for (mut timer, mut block, mut transform, movable) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if movable.can_down {
                block.y -= 1;
                transform.translation = block.translation();
                audio.play(game_audios.drop.clone());
            }
        }
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
        let piece = match query.iter().next() {
            Some((piece, _, _)) => piece.clone(),
            None => {
                return;
            }
        };
        let mut blocks: Vec<Mut<Block>> = Vec::new();
        let mut transforms: Vec<Mut<Transform>> = Vec::new();
        for (_, block, transform) in &mut query {
            blocks.push(block);
            transforms.push(transform);
        }
        let mut piece_config = PieceConfig::new(
            piece.clone(),
            [*blocks[0], *blocks[1], *blocks[2], *blocks[3]],
        );
        match piece {
            PieceType::I(angle) => {
                piece_config = rotate_piece_i(piece_config);
            }
            PieceType::J(angle) => {
                piece_config = rotate_piece_j(piece_config);
            }
            PieceType::L(angle) => {
                piece_config = rotate_piece_l(piece_config);
            }
            PieceType::O(angle) => {}
            PieceType::S(angle) => {
                piece_config = rotate_piece_s(piece_config);
            }
            PieceType::T(angle) => {
                piece_config = rotate_piece_t(piece_config);
            }
            PieceType::Z(angle) => {
                piece_config = rotate_piece_z(piece_config);
            }
        }

        // 更新block位置
        blocks
            .get_mut(0)
            .unwrap()
            .set(piece_config.blocks[0].x, piece_config.blocks[0].y);
        blocks
            .get_mut(1)
            .unwrap()
            .set(piece_config.blocks[1].x, piece_config.blocks[1].y);
        blocks
            .get_mut(2)
            .unwrap()
            .set(piece_config.blocks[2].x, piece_config.blocks[2].y);
        blocks
            .get_mut(3)
            .unwrap()
            .set(piece_config.blocks[3].x, piece_config.blocks[3].y);
        transforms.get_mut(0).unwrap().translation = blocks.get(0).unwrap().translation();
        transforms.get_mut(1).unwrap().translation = blocks.get(1).unwrap().translation();
        transforms.get_mut(2).unwrap().translation = blocks.get(2).unwrap().translation();
        transforms.get_mut(3).unwrap().translation = blocks.get(3).unwrap().translation();

        // 更新piece角度
        for (mut piece, _, _) in &mut query {
            *piece = piece_config.piece;
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

fn rotate_piece_i(piece_config: PieceConfig) -> PieceConfig {
    let min_x = blocks_min_x(&piece_config.blocks);
    let max_x = blocks_max_x(&piece_config.blocks);
    let min_y = blocks_min_y(&piece_config.blocks);
    let max_y = blocks_max_y(&piece_config.blocks);
    match piece_config.piece {
        PieceType::I(RotationAngle::Angle0) | PieceType::I(RotationAngle::Angle180) => {
            // 当前为水平方向
            let new_x = max_x - 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::I(RotationAngle::Angle90),
                [
                    Block {
                        x: new_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_x,
                        y: new_min_y + 2,
                    },
                    Block {
                        x: new_x,
                        y: new_min_y + 3,
                    },
                ],
            );
        }
        PieceType::I(RotationAngle::Angle90) | PieceType::I(RotationAngle::Angle270) => {
            // 当前为垂直方向
            let new_y = max_y - 1;
            let new_min_x = min_x - 1;
            return PieceConfig::new(
                PieceType::I(RotationAngle::Angle0),
                [
                    Block {
                        x: new_min_x,
                        y: new_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_y,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_y,
                    },
                    Block {
                        x: new_min_x + 3,
                        y: new_y,
                    },
                ],
            );
        }
        _ => {
            panic!("No matched piece for Piece::I");
        }
    }
}

fn rotate_piece_j(piece_config: PieceConfig) -> PieceConfig {
    let min_x = blocks_min_x(&piece_config.blocks);
    let max_x = blocks_max_x(&piece_config.blocks);
    let min_y = blocks_min_y(&piece_config.blocks);
    let max_y = blocks_max_y(&piece_config.blocks);
    match piece_config.piece {
        PieceType::J(RotationAngle::Angle0) => {
            let new_min_x = min_x;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::J(RotationAngle::Angle90),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 2,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 2,
                    },
                ],
            );
        }
        PieceType::J(RotationAngle::Angle90) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                PieceType::J(RotationAngle::Angle180),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y,
                    },
                ],
            );
        }
        PieceType::J(RotationAngle::Angle180) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::J(RotationAngle::Angle270),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 2,
                    },
                ],
            );
        }
        PieceType::J(RotationAngle::Angle270) => {
            let new_min_x = min_x - 1;
            let new_min_y = min_y;
            return PieceConfig::new(
                PieceType::J(RotationAngle::Angle0),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y,
                    },
                ],
            );
        }
        _ => {
            panic!("No matched piece for Piece::J");
        }
    }
}
fn rotate_piece_l(piece_config: PieceConfig) -> PieceConfig {
    let min_x = blocks_min_x(&piece_config.blocks);
    let max_x = blocks_max_x(&piece_config.blocks);
    let min_y = blocks_min_y(&piece_config.blocks);
    let max_y = blocks_max_y(&piece_config.blocks);
    match piece_config.piece {
        PieceType::L(RotationAngle::Angle0) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::L(RotationAngle::Angle90),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 2,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                ],
            );
        }
        PieceType::L(RotationAngle::Angle90) => {
            let new_min_x = min_x;
            let new_min_y = min_y;
            return PieceConfig::new(
                PieceType::L(RotationAngle::Angle180),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y + 1,
                    },
                ],
            );
        }
        PieceType::L(RotationAngle::Angle180) => {
            let new_min_x = min_x;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::L(RotationAngle::Angle270),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y + 2,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 2,
                    },
                ],
            );
        }
        PieceType::L(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                PieceType::L(RotationAngle::Angle0),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y + 1,
                    },
                ],
            );
        }
        _ => {
            panic!("No matched piece for Piece::L");
        }
    }
}
fn rotate_piece_s(piece_config: PieceConfig) -> PieceConfig {
    let min_x = blocks_min_x(&piece_config.blocks);
    let max_x = blocks_max_x(&piece_config.blocks);
    let min_y = blocks_min_y(&piece_config.blocks);
    let max_y = blocks_max_y(&piece_config.blocks);
    match piece_config.piece {
        PieceType::S(RotationAngle::Angle0) | PieceType::S(RotationAngle::Angle180) => {
            let new_min_x = min_x;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::S(RotationAngle::Angle90),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 2,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                ],
            );
        }
        PieceType::S(RotationAngle::Angle90) | PieceType::S(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                PieceType::S(RotationAngle::Angle0),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y + 1,
                    },
                ],
            );
        }
        _ => {
            panic!("No matched piece for Piece::S");
        }
    }
}
fn rotate_piece_t(piece_config: PieceConfig) -> PieceConfig {
    let min_x = blocks_min_x(&piece_config.blocks);
    let max_x = blocks_max_x(&piece_config.blocks);
    let min_y = blocks_min_y(&piece_config.blocks);
    let max_y = blocks_max_y(&piece_config.blocks);
    match piece_config.piece {
        PieceType::T(RotationAngle::Angle0) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::T(RotationAngle::Angle90),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 2,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                ],
            );
        }
        PieceType::T(RotationAngle::Angle90) => {
            let new_min_x = min_x - 1;
            let new_min_y = min_y;
            return PieceConfig::new(
                PieceType::T(RotationAngle::Angle180),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y + 1,
                    },
                ],
            );
        }
        PieceType::T(RotationAngle::Angle180) => {
            let new_min_x = min_x;
            let new_min_y = min_y;
            return PieceConfig::new(
                PieceType::T(RotationAngle::Angle270),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 2,
                    },
                ],
            );
        }
        PieceType::T(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                PieceType::T(RotationAngle::Angle0),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y,
                    },
                ],
            );
        }
        _ => {
            panic!("No matched piece for Piece::T");
        }
    }
}
fn rotate_piece_z(piece_config: PieceConfig) -> PieceConfig {
    let min_x = blocks_min_x(&piece_config.blocks);
    let max_x = blocks_max_x(&piece_config.blocks);
    let min_y = blocks_min_y(&piece_config.blocks);
    let max_y = blocks_max_y(&piece_config.blocks);
    match piece_config.piece {
        PieceType::Z(RotationAngle::Angle0) | PieceType::Z(RotationAngle::Angle180) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                PieceType::Z(RotationAngle::Angle90),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 2,
                    },
                ],
            );
        }
        PieceType::Z(RotationAngle::Angle90) | PieceType::Z(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y;
            return PieceConfig::new(
                PieceType::Z(RotationAngle::Angle0),
                [
                    Block {
                        x: new_min_x,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y,
                    },
                    Block {
                        x: new_min_x + 1,
                        y: new_min_y + 1,
                    },
                    Block {
                        x: new_min_x + 2,
                        y: new_min_y,
                    },
                ],
            );
        }
        _ => {
            panic!("No matched piece for Piece::Z");
        }
    }
}

// 自动生成新的四格骨牌
pub fn auto_generate_new_piece(
    mut commands: Commands,
    query: Query<&PieceType>,
    game_over_events: EventReader<GameOverEvent>,
    mut piece_queue: ResMut<PieceQueue>,
) {
    if !game_over_events.is_empty() {
        return;
    }
    if piece_queue.0.len() < PieceType::PIECE_AMOUNT as usize {
        piece_queue.0.extend(random_7_pieces());
    }
    if query.is_empty() {
        let piece_config = piece_queue.0.pop_front().unwrap();
        // 生成新的四格骨牌
        let color = piece_config.color;
        let block = piece_config.blocks[0].clone();
        let visibility = Visibility::Hidden;
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_block_sprite(&block, color, visibility))
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
        let block = piece_config.blocks[1].clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_block_sprite(&block, color, visibility))
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
        let block = piece_config.blocks[2].clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_block_sprite(&block, color, visibility))
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
        let block = piece_config.blocks[3].clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_block_sprite(&block, color, visibility))
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

// bag7算法实现随机：每次填充7个随机排序的骨牌
fn random_7_pieces() -> Vec<PieceConfig> {
    let mut result: Vec<PieceConfig> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut rand_ints: Vec<u32> = Vec::new();

    loop {
        let mut select = |rand_int: u32| {
            if !rand_ints.contains(&rand_int) {
                rand_ints.push(rand_int);
                result.push(ALL_PIECES.get(rand_int as usize).unwrap().clone());
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
