#![allow(clippy::explicit_counter_loop)]

use itertools::Itertools;

use crate::game::{BlockShapeCells, Cell, Point};

#[derive(Debug, Clone)]
pub struct Board {
    pub column_count: u32,     // 열 개수(가로 길이)
    pub row_count: u32,        // 행 개수(세로 길이)
    pub hidden_row_count: u32, // 숨겨진 상단 행 개수
    pub board_width: u32,
    pub board_height: u32,
    pub cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn unfold(&self) -> Vec<i32> {
        self.cells
            .clone()
            .into_iter()
            .flatten()
            .map(|e| e.into_code())
            .collect::<Vec<_>>()
    }

    pub fn from_unfold(
        unfolded: Vec<i32>,
        board_width: u32,
        board_height: u32,
        column_count: u32,
        row_count: u32,
        hidden_row_count: u32,
    ) -> Self {
        Self {
            column_count,
            row_count,
            board_width,
            board_height,
            hidden_row_count,
            cells: unfolded
                .into_iter()
                .map(|e| Cell::try_from(e).unwrap())
                .chunks(column_count as usize)
                .into_iter()
                .map(|chunk| chunk.collect::<Vec<Cell>>())
                .collect(),
        }
    }

    pub fn write_current_block(&mut self, block: BlockShapeCells, position: Point) {
        let x = position.x;
        let y = position.y;

        let mut block_x = 0;

        let block_row_count = block.len();
        let block_column_count = block[0].len();

        for x in x..(x + block_column_count as i64) {
            let mut block_y = 0;

            for y in y..(y + block_row_count as i64) {
                let y = y as usize;
                let x = x as usize;

                let cell = self.cells.get(y).map(|e| e.get(x)).flatten();

                match cell {
                    Some(cell) => {
                        if let Cell::Empty = cell {
                            // No Conflict
                            self.cells[y][x] = block[block_y][block_x];
                        } else if let Cell::Ghost = cell {
                            // No Conflict
                            self.cells[y][x] = block[block_y][block_x];
                        } else if let Cell::Empty = block[block_y][block_x] {
                            // No Conflict
                        } else if let Cell::Ghost = block[block_y][block_x] {
                            // No Conflict
                        } else {
                            // Conflict
                            panic!("block conflict");
                        }
                    }
                    None => {}
                }

                block_y += 1;
            }

            block_x += 1;
        }
    }
}
