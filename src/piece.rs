use crate::board::*;
use bevy::prelude::*;
use rand::Rng;

// 四格骨牌
#[derive(Component, Debug, Clone)]
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
    piece: Piece,
    init_blocks: Piece4Blocks,
    color: Color,
}

pub struct Piece4Blocks(Block, Block, Block, Block);

impl Piece4Blocks {
    pub fn from_vec(blocks: &Vec<Mut<Block>>) -> Self {
        Piece4Blocks(
            blocks.get(0).unwrap().as_ref().clone(),
            blocks.get(1).unwrap().as_ref().clone(),
            blocks.get(2).unwrap().as_ref().clone(),
            blocks.get(3).unwrap().as_ref().clone(),
        )
    }
    pub fn min_x(&self) -> u32 {
        std::cmp::min(self.0.x, self.1.x).min(self.2.x).min(self.3.x)
    }
    pub fn max_x(&self) -> u32 {
        std::cmp::max(self.0.x, self.1.x).max(self.2.x).max(self.3.x)
    }
    pub fn min_y(&self) -> u32 {
        std::cmp::min(self.0.y, self.1.y).min(self.2.y).min(self.3.y)
    }
    pub fn max_y(&self) -> u32 {
        std::cmp::max(self.0.y, self.1.y).max(self.2.x).max(self.3.x)
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
#[derive(Component, Deref, DerefMut)]
pub struct AutoMovePieceDownTimer(pub Timer);

// 手动移动四格骨牌
pub fn manually_move_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Block, &mut Transform, &Movable), With<Piece>>,
) {
    if keyboard_input.pressed(KeyCode::Left) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_left {
                block.x -= 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.pressed(KeyCode::Right) {
        for (mut block, mut transform, movable) in &mut query {
            if movable.can_right {
                block.x += 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.pressed(KeyCode::Down) {
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

// TODO 旋转四格骨牌
// TODO 旋转不能产生碰撞（进入物体）
pub fn rotate_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Piece, &mut Block, &mut Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::Up) {
        let piece = match query.iter().next() {
            Some((piece, _, _)) => piece,
            None => {
                return;
            }
        };
        match piece {
            Piece::I => {
                let mut blocks: Vec<Mut<Block>> = Vec::new();
                let mut transforms: Vec<Mut<Transform>> = Vec::new();
                for (_, block, transform) in &mut query {
                    blocks.push(block);
                    transforms.push(transform);
                }
                let new_blocks = rotate_piece_i(Piece4Blocks::from_vec(&blocks));
                blocks.get_mut(0).unwrap().set(new_blocks.0.x, new_blocks.0.y);
                blocks.get_mut(1).unwrap().set(new_blocks.1.x, new_blocks.1.y);
                blocks.get_mut(2).unwrap().set(new_blocks.2.x, new_blocks.2.y);
                blocks.get_mut(3).unwrap().set(new_blocks.3.x, new_blocks.3.y);
                transforms.get_mut(0).unwrap().translation = blocks.get(0).unwrap().translation();
                transforms.get_mut(1).unwrap().translation = blocks.get(1).unwrap().translation();
                transforms.get_mut(2).unwrap().translation = blocks.get(2).unwrap().translation();
                transforms.get_mut(3).unwrap().translation = blocks.get(3).unwrap().translation();
            }
            Piece::J => {}
            Piece::L => {}
            Piece::O => {}
            Piece::S => {}
            Piece::T => {}
            Piece::Z => {}
        }
    }
}

fn rotate_piece_i(blocks: Piece4Blocks) -> Piece4Blocks {
    let min_x = blocks.min_x();
    let max_x = blocks.max_x();
    let min_y = blocks.min_y();
    let max_y = blocks.max_y();
    if min_x == max_x {
        // 当前为垂直方向
        let new_y = max_y as u32 - 1;
        let new_min_x = min_x as u32 - 1;
        return Piece4Blocks(
            Block { x: new_min_x, y: new_y },
            Block { x: new_min_x + 1, y: new_y },
            Block { x: new_min_x + 2, y: new_y },
            Block { x: new_min_x + 3, y: new_y },
        );
    } else {
        // 当前为水平方向
        let new_x = max_x as u32 - 1;
        let new_min_y = min_y as u32 - 1;
        return Piece4Blocks(
            Block { x: new_x, y: new_min_y },
            Block { x: new_x, y: new_min_y + 1},
            Block { x: new_x, y: new_min_y + 2},
            Block { x: new_x, y: new_min_y + 3},
        );
    }
}

// 自动生成新的四格骨牌
pub fn auto_generate_new_piece(
    mut commands: Commands,
    query: Query<&Piece>,
    game_over_events: EventReader<GameOverEvent>,
) {
    if !game_over_events.is_empty() {
        return;
    }
    if query.is_empty() {
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
            .spawn(piece_config.piece.clone())
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
            .spawn(piece_config.piece.clone())
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
            .spawn(piece_config.piece.clone())
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
            .spawn(piece_config.piece.clone())
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
                piece: Piece::I,
                init_blocks: Piece4Blocks(
                    Block::new(3, 19),
                    Block::new(4, 19),
                    Block::new(5, 19),
                    Block::new(6, 19),
                ),
                color: Color::BLACK,
            }
        }
        1 => {
            // Piece::J
            PieceConfig {
                piece: Piece::J,
                init_blocks: Piece4Blocks(
                    Block::new(4, 20),
                    Block::new(4, 19),
                    Block::new(5, 19),
                    Block::new(6, 19),
                ),
                color: Color::BLACK,
            }
        }
        2 => {
            // Piece::L
            PieceConfig {
                piece: Piece::L,
                init_blocks: Piece4Blocks(
                    Block::new(3, 19),
                    Block::new(4, 19),
                    Block::new(5, 19),
                    Block::new(5, 20),
                ),
                color: Color::BLACK,
            }
        }
        3 => {
            // Piece::O
            PieceConfig {
                piece: Piece::O,
                init_blocks: Piece4Blocks(
                    Block::new(4, 20),
                    Block::new(4, 19),
                    Block::new(5, 19),
                    Block::new(5, 20),
                ),
                color: Color::BLACK,
            }
        }
        4 => {
            // Piece::S
            PieceConfig {
                piece: Piece::S,
                init_blocks: Piece4Blocks(
                    Block::new(3, 19),
                    Block::new(4, 19),
                    Block::new(4, 20),
                    Block::new(5, 20),
                ),
                color: Color::BLACK,
            }
        }
        5 => {
            // Piece::T
            PieceConfig {
                piece: Piece::T,
                init_blocks: Piece4Blocks(
                    Block::new(3, 19),
                    Block::new(4, 20),
                    Block::new(4, 19),
                    Block::new(5, 19),
                ),
                color: Color::BLACK,
            }
        }
        6 => {
            // Piece::Z
            PieceConfig {
                piece: Piece::Z,
                init_blocks: Piece4Blocks(
                    Block::new(4, 20),
                    Block::new(5, 20),
                    Block::new(5, 19),
                    Block::new(6, 19),
                ),
                color: Color::BLACK,
            }
        }
        _ => {
            panic!("No matched piece")
        }
    }
}
