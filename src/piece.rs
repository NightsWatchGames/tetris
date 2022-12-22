use crate::board::*;
use bevy::prelude::*;
use rand::Rng;

// 四格骨牌
#[derive(Component, Debug, Clone, Copy)]
pub enum Piece {
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

// 旋转角度
#[derive(Debug, Clone, Copy)]
pub enum RotationAngle {
    Angle0,
    Angle90,
    Angle180,
    Angle270,
}

#[derive(Debug)]
pub struct PieceConfig {
    pub piece: Piece,
    pub blocks: Piece4Blocks,
    pub color: Color,
}

impl PieceConfig {
    pub fn new(piece: Piece, blocks: Piece4Blocks) -> Self {
        let color = match piece {
            Piece::I(_) => Color::CYAN,
            Piece::J(_) => Color::BLUE,
            Piece::L(_) => Color::ORANGE,
            Piece::O(_) => Color::YELLOW,
            Piece::S(_) => Color::GREEN,
            Piece::T(_) => Color::PURPLE,
            Piece::Z(_) => Color::RED,
        };
        PieceConfig {
            piece,
            blocks,
            color,
        }
    }
}

#[derive(Debug)]
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
        std::cmp::min(self.0.x, self.1.x)
            .min(self.2.x)
            .min(self.3.x)
    }
    pub fn max_x(&self) -> u32 {
        std::cmp::max(self.0.x, self.1.x)
            .max(self.2.x)
            .max(self.3.x)
    }
    pub fn min_y(&self) -> u32 {
        std::cmp::min(self.0.y, self.1.y)
            .min(self.2.y)
            .min(self.3.y)
    }
    pub fn max_y(&self) -> u32 {
        std::cmp::max(self.0.y, self.1.y)
            .max(self.2.x)
            .max(self.3.x)
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
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Block, &mut Transform, &Movable), With<Piece>>,
) {
    if keyboard_input.pressed(KeyCode::Left) {
        for (_, mut block, mut transform, movable) in &mut query {
            if movable.can_left {
                block.x -= 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.pressed(KeyCode::Right) {
        for (_, mut block, mut transform, movable) in &mut query {
            if movable.can_right {
                block.x += 1;
                transform.translation = block.translation();
            }
        }
    } else if keyboard_input.pressed(KeyCode::Down) {
        for (entity, mut block, mut transform, movable) in &mut query {
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
        With<Piece>,
    >,
) {
    for (mut timer, mut block, mut transform, movable) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if movable.can_down {
                block.y -= 1;
                transform.translation = block.translation();
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

// TODO 旋转不能产生碰撞（进入物体）
pub fn rotate_piece(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Piece, &mut Block, &mut Transform)>,
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
        let mut piece_config = PieceConfig::new(piece.clone(), Piece4Blocks::from_vec(&blocks));
        match piece {
            Piece::I(angle) => {
                piece_config = rotate_piece_i(piece_config);
            }
            Piece::J(angle) => {
                piece_config = rotate_piece_j(piece_config);
            }
            Piece::L(angle) => {
                piece_config = rotate_piece_l(piece_config);
            }
            Piece::O(angle) => {}
            Piece::S(angle) => {
                piece_config = rotate_piece_s(piece_config);
            }
            Piece::T(angle) => {
                piece_config = rotate_piece_t(piece_config);
            }
            Piece::Z(angle) => {
                piece_config = rotate_piece_z(piece_config);
            }
        }

        // 更新block位置
        blocks
            .get_mut(0)
            .unwrap()
            .set(piece_config.blocks.0.x, piece_config.blocks.0.y);
        blocks
            .get_mut(1)
            .unwrap()
            .set(piece_config.blocks.1.x, piece_config.blocks.1.y);
        blocks
            .get_mut(2)
            .unwrap()
            .set(piece_config.blocks.2.x, piece_config.blocks.2.y);
        blocks
            .get_mut(3)
            .unwrap()
            .set(piece_config.blocks.3.x, piece_config.blocks.3.y);
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

fn rotate_piece_i(piece_config: PieceConfig) -> PieceConfig {
    let min_x = piece_config.blocks.min_x();
    let max_x = piece_config.blocks.max_x();
    let min_y = piece_config.blocks.min_y();
    let max_y = piece_config.blocks.max_y();
    match piece_config.piece {
        Piece::I(RotationAngle::Angle0) | Piece::I(RotationAngle::Angle180) => {
            // 当前为水平方向
            let new_x = max_x as u32 - 1;
            let new_min_y = min_y as u32 - 1;
            return PieceConfig::new(
                Piece::I(RotationAngle::Angle90),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::I(RotationAngle::Angle90) | Piece::I(RotationAngle::Angle270) => {
            // 当前为垂直方向
            let new_y = max_y as u32 - 1;
            let new_min_x = min_x as u32 - 1;
            return PieceConfig::new(
                Piece::I(RotationAngle::Angle0),
                Piece4Blocks(
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
                ),
            );
        }
        _ => {
            panic!("No matched piece for Piece::I");
        }
    }
}

fn rotate_piece_j(piece_config: PieceConfig) -> PieceConfig {
    let min_x = piece_config.blocks.min_x();
    let max_x = piece_config.blocks.max_x();
    let min_y = piece_config.blocks.min_y();
    let max_y = piece_config.blocks.max_y();
    match piece_config.piece {
        Piece::J(RotationAngle::Angle0) => {
            let new_min_x = min_x;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::J(RotationAngle::Angle90),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::J(RotationAngle::Angle90) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                Piece::J(RotationAngle::Angle180),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::J(RotationAngle::Angle180) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::J(RotationAngle::Angle270),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::J(RotationAngle::Angle270) => {
            let new_min_x = min_x - 1;
            let new_min_y = min_y;
            return PieceConfig::new(
                Piece::J(RotationAngle::Angle0),
                Piece4Blocks(
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
                ),
            );
        }
        _ => {
            panic!("No matched piece for Piece::J");
        }
    }
}
fn rotate_piece_l(piece_config: PieceConfig) -> PieceConfig {
    let min_x = piece_config.blocks.min_x();
    let max_x = piece_config.blocks.max_x();
    let min_y = piece_config.blocks.min_y();
    let max_y = piece_config.blocks.max_y();
    match piece_config.piece {
        Piece::L(RotationAngle::Angle0) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::L(RotationAngle::Angle90),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::L(RotationAngle::Angle90) => {
            let new_min_x = min_x;
            let new_min_y = min_y;
            return PieceConfig::new(
                Piece::L(RotationAngle::Angle180),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::L(RotationAngle::Angle180) => {
            let new_min_x = min_x;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::L(RotationAngle::Angle270),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::L(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                Piece::L(RotationAngle::Angle0),
                Piece4Blocks(
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
                ),
            );
        }
        _ => {
            panic!("No matched piece for Piece::L");
        }
    }
}
fn rotate_piece_s(piece_config: PieceConfig) -> PieceConfig {
    let min_x = piece_config.blocks.min_x();
    let max_x = piece_config.blocks.max_x();
    let min_y = piece_config.blocks.min_y();
    let max_y = piece_config.blocks.max_y();
    match piece_config.piece {
        Piece::S(RotationAngle::Angle0) | Piece::S(RotationAngle::Angle180) => {
            let new_min_x = min_x;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::S(RotationAngle::Angle90),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::S(RotationAngle::Angle90) | Piece::S(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                Piece::S(RotationAngle::Angle0),
                Piece4Blocks(
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
                ),
            );
        }
        _ => {
            panic!("No matched piece for Piece::S");
        }
    }
}
fn rotate_piece_t(piece_config: PieceConfig) -> PieceConfig {
    let min_x = piece_config.blocks.min_x();
    let max_x = piece_config.blocks.max_x();
    let min_y = piece_config.blocks.min_y();
    let max_y = piece_config.blocks.max_y();
    match piece_config.piece {
        Piece::T(RotationAngle::Angle0) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::T(RotationAngle::Angle90),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::T(RotationAngle::Angle90) => {
            let new_min_x = min_x - 1;
            let new_min_y = min_y;
            return PieceConfig::new(
                Piece::T(RotationAngle::Angle180),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::T(RotationAngle::Angle180) => {
            let new_min_x = min_x;
            let new_min_y = min_y;
            return PieceConfig::new(
                Piece::T(RotationAngle::Angle270),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::T(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y + 1;
            return PieceConfig::new(
                Piece::T(RotationAngle::Angle0),
                Piece4Blocks(
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
                ),
            );
        }
        _ => {
            panic!("No matched piece for Piece::T");
        }
    }
}
fn rotate_piece_z(piece_config: PieceConfig) -> PieceConfig {
    let min_x = piece_config.blocks.min_x();
    let max_x = piece_config.blocks.max_x();
    let min_y = piece_config.blocks.min_y();
    let max_y = piece_config.blocks.max_y();
    match piece_config.piece {
        Piece::Z(RotationAngle::Angle0) | Piece::Z(RotationAngle::Angle180) => {
            let new_min_x = min_x + 1;
            let new_min_y = min_y - 1;
            return PieceConfig::new(
                Piece::Z(RotationAngle::Angle90),
                Piece4Blocks(
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
                ),
            );
        }
        Piece::Z(RotationAngle::Angle90) | Piece::Z(RotationAngle::Angle270) => {
            let new_min_x = min_x;
            let new_min_y = min_y;
            return PieceConfig::new(
                Piece::Z(RotationAngle::Angle0),
                Piece4Blocks(
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
                ),
            );
        }
        _ => {
            panic!("No matched piece for Piece::Z");
        }
    }
}

// 自动生成新的四格骨牌
// TODO 超出game board的不显示
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
        let new_sprite_bundle = |block: &Block, color: Color| SpriteBundle {
            sprite: Sprite { color, ..default() },
            transform: Transform {
                scale: Vec3::new(BLOCK_STICKER_LENGTH, BLOCK_STICKER_LENGTH, BLOCK_STICKER_LENGTH),
                translation: block.translation(),
                ..default()
            },
            ..default()
        };
        let color = piece_config.color;
        let block = piece_config.blocks.0.clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_sprite_bundle(&block, color))
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
        let block = piece_config.blocks.1.clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_sprite_bundle(&block, color))
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
        let block = piece_config.blocks.2.clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_sprite_bundle(&block, color))
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
        let block = piece_config.blocks.3.clone();
        commands
            .spawn(piece_config.piece.clone())
            .insert(new_sprite_bundle(&block, color))
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
        0 => PieceConfig::new(
            Piece::I(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(3, 19),
                Block::new(4, 19),
                Block::new(5, 19),
                Block::new(6, 19),
            ),
        ),
        1 => PieceConfig::new(
            Piece::J(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(4, 20),
                Block::new(4, 19),
                Block::new(5, 19),
                Block::new(6, 19),
            ),
        ),
        2 => PieceConfig::new(
            Piece::L(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(3, 19),
                Block::new(4, 19),
                Block::new(5, 19),
                Block::new(5, 20),
            ),
        ),
        3 => PieceConfig::new(
            Piece::O(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(4, 20),
                Block::new(4, 19),
                Block::new(5, 19),
                Block::new(5, 20),
            ),
        ),
        4 => PieceConfig::new(
            Piece::S(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(3, 19),
                Block::new(4, 19),
                Block::new(4, 20),
                Block::new(5, 20),
            ),
        ),
        5 => PieceConfig::new(
            Piece::T(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(3, 19),
                Block::new(4, 20),
                Block::new(4, 19),
                Block::new(5, 19),
            ),
        ),
        6 => PieceConfig::new(
            Piece::Z(RotationAngle::Angle0),
            Piece4Blocks(
                Block::new(4, 20),
                Block::new(5, 20),
                Block::new(5, 19),
                Block::new(6, 19),
            ),
        ),
        _ => {
            panic!("No matched piece")
        }
    }
}
