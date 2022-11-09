use serde::{Deserialize, Serialize};

use crate::game::cell::Cell;

use super::colors::{
    I_DEFAULT_COLOR, J_DEFAULT_COLOR, L_DEFAULT_COLOR, O_DEFAULT_COLOR, S_DEFAULT_COLOR,
    T_DEFAULT_COLOR, Z_DEFAULT_COLOR,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub enum Block {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
    #[default]
    ETC,
}

impl From<Block> for i32 {
    fn from(value: Block) -> Self {
        match value {
            Block::I => 0,
            Block::J => 1,
            Block::L => 2,
            Block::O => 3,
            Block::S => 4,
            Block::T => 5,
            Block::Z => 6,
            Block::ETC => 99,
        }
    }
}

impl From<i32> for Block {
    fn from(value: i32) -> Self {
        match value {
            0 => Block::I,
            1 => Block::J,
            2 => Block::L,
            3 => Block::O,
            4 => Block::S,
            5 => Block::T,
            6 => Block::Z,
            _ => Block::ETC,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BlockShape {
    pub block: Block,
    pub cells: BlockShapeCells,
    pub rotation_count: usize,
}

pub type BlockShapeCells = [[Cell; 4]; 4];

impl From<i32> for BlockShape {
    fn from(value: i32) -> Self {
        match value {
            0 => BlockShape::I,
            1 => BlockShape::J,
            2 => BlockShape::L,
            3 => BlockShape::O,
            4 => BlockShape::S,
            5 => BlockShape::T,
            6 => BlockShape::Z,
            99 => BlockShape::NONE,
            _ => BlockShape::NONE,
        }
    }
}

impl BlockShape {
    pub fn to_ghost(mut self) -> Self {
        for row in &mut self.cells {
            for cell in row {
                if !cell.is_empty() {
                    *cell = Cell::Ghost;
                }
            }
        }

        self
    }
}

impl BlockShape {
    // □□□□
    // ■■■■
    // □□□□
    // □□□□
    pub const I: Self = Self {
        block: Block::I,
        rotation_count: 0,
        cells: [
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [
                I_DEFAULT_COLOR,
                I_DEFAULT_COLOR,
                I_DEFAULT_COLOR,
                I_DEFAULT_COLOR,
            ],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // □□■□
    // ■■■□
    // □□□□
    // □□□□
    pub const L: Self = Self {
        block: Block::L,
        rotation_count: 0,

        cells: [
            [Cell::Empty, Cell::Empty, L_DEFAULT_COLOR, Cell::Empty],
            [
                L_DEFAULT_COLOR,
                L_DEFAULT_COLOR,
                L_DEFAULT_COLOR,
                Cell::Empty,
            ],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // ■□□□
    // ■■■□
    // □□□□
    // □□□□
    pub const J: BlockShape = Self {
        block: Block::J,
        rotation_count: 0,

        cells: [
            [J_DEFAULT_COLOR, Cell::Empty, Cell::Empty, Cell::Empty],
            [
                J_DEFAULT_COLOR,
                J_DEFAULT_COLOR,
                J_DEFAULT_COLOR,
                Cell::Empty,
            ],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // □■■□
    // □■■□
    // □□□□
    // □□□□
    pub const O: Self = Self {
        block: Block::O,
        rotation_count: 0,

        cells: [
            [Cell::Empty, O_DEFAULT_COLOR, O_DEFAULT_COLOR, Cell::Empty],
            [Cell::Empty, O_DEFAULT_COLOR, O_DEFAULT_COLOR, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // □■■□
    // ■■□□
    // □□□□
    // □□□□
    pub const S: Self = Self {
        block: Block::S,
        rotation_count: 0,

        cells: [
            [Cell::Empty, S_DEFAULT_COLOR, S_DEFAULT_COLOR, Cell::Empty],
            [S_DEFAULT_COLOR, S_DEFAULT_COLOR, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // ■■□□
    // □■■□
    // □□□□
    // □□□□
    pub const Z: Self = Self {
        block: Block::Z,
        rotation_count: 0,

        cells: [
            [Z_DEFAULT_COLOR, Z_DEFAULT_COLOR, Cell::Empty, Cell::Empty],
            [Cell::Empty, Z_DEFAULT_COLOR, Z_DEFAULT_COLOR, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // □■□□
    // ■■■□
    // □□□□
    // □□□□
    pub const T: Self = Self {
        block: Block::T,
        rotation_count: 0,

        cells: [
            [Cell::Empty, T_DEFAULT_COLOR, Cell::Empty, Cell::Empty],
            [
                T_DEFAULT_COLOR,
                T_DEFAULT_COLOR,
                T_DEFAULT_COLOR,
                Cell::Empty,
            ],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };

    // □□□□
    // □□□□
    // □□□□
    // □□□□
    pub const NONE: Self = Self {
        block: Block::ETC,
        rotation_count: 0,

        cells: [
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
            [Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty],
        ],
    };
}
