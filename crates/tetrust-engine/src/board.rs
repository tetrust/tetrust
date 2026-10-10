//! 보드. 너비 10, 높이 40(보이는 20줄 + 버퍼 20줄), 0번 행이 맨 아래.

use crate::piece::{ActivePiece, PieceKind};

pub const WIDTH: i32 = 10;
pub const HEIGHT: i32 = 40;
pub const VISIBLE_HEIGHT: i32 = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Cell {
    #[default]
    Empty,
    Piece(PieceKind),
    Garbage,
}

impl Cell {
    pub fn is_empty(self) -> bool {
        self == Cell::Empty
    }
}

pub type Row = [Cell; WIDTH as usize];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    rows: Vec<Row>,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            rows: vec![[Cell::Empty; WIDTH as usize]; HEIGHT as usize],
        }
    }
}

impl Board {
    /// 테스트·디버그용. 위에서 아래 순서의 문자열로 아래쪽 행들을 채운다.
    /// `.`은 빈 칸, 그 밖의 문자는 가비지 칸이다.
    pub fn from_rows(rows: &[&str]) -> Self {
        let mut board = Self::default();
        for (i, line) in rows.iter().rev().enumerate() {
            assert_eq!(line.len(), WIDTH as usize, "row must be {WIDTH} wide");
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    board.rows[i][x] = Cell::Garbage;
                }
            }
        }
        board
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if !(0..WIDTH).contains(&x) || !(0..HEIGHT).contains(&y) {
            return None;
        }
        Some(self.rows[y as usize][x as usize])
    }

    /// 보드 밖(좌우·아래·위)이거나 채워진 칸이면 true.
    pub fn is_blocked(&self, x: i32, y: i32) -> bool {
        self.get(x, y).is_none_or(|c| !c.is_empty())
    }

    pub fn fits(&self, piece: &ActivePiece) -> bool {
        piece.cells().iter().all(|&(x, y)| !self.is_blocked(x, y))
    }

    pub fn rows(&self) -> &[Row] {
        &self.rows
    }

    pub fn place(&mut self, piece: &ActivePiece) {
        for (x, y) in piece.cells() {
            self.rows[y as usize][x as usize] = Cell::Piece(piece.kind);
        }
    }

    /// 꽉 찬 줄을 지우고 지운 줄 수를 돌려준다.
    pub fn clear_lines(&mut self) -> u32 {
        let before = self.rows.len();
        self.rows.retain(|row| row.iter().any(|c| c.is_empty()));
        let cleared = before - self.rows.len();
        self.rows
            .resize(HEIGHT as usize, [Cell::Empty; WIDTH as usize]);
        cleared as u32
    }

    pub fn is_empty(&self) -> bool {
        self.rows.iter().all(|row| row.iter().all(|c| c.is_empty()))
    }

    /// 아래에서 가비지 줄을 밀어 올린다. 맨 위 밖으로 블록이 밀려나면 false.
    pub fn insert_garbage(&mut self, lines: u32, hole: i32) -> bool {
        let lines = lines as usize;
        let overflow = self.rows[HEIGHT as usize - lines..]
            .iter()
            .any(|row| row.iter().any(|c| !c.is_empty()));
        self.rows.truncate(HEIGHT as usize - lines);
        let mut garbage = [Cell::Garbage; WIDTH as usize];
        garbage[hole as usize] = Cell::Empty;
        for _ in 0..lines {
            self.rows.insert(0, garbage);
        }
        !overflow
    }
}
