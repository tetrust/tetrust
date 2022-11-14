use crate::game::{BlockShape, BlockShapeCells, Board, Cell, Point, SpinType};

// 블럭 충돌여부 검증
pub fn valid_block(board: &Board, block: &BlockShapeCells, point: Point) -> bool {
    let block_row_count = block.len();
    let block_column_count = block[0].len();

    let column_count = board.column_count as usize;
    let center_index = column_count / 2;
    let above_full = board.cells[0][center_index - 2..center_index + 2]
        .iter()
        .any(|e| !e.is_empty());

    for (block_x, x) in (point.x..(point.x + block_row_count as i64)).enumerate() {
        for (block_y, y) in (point.y..(point.y + block_column_count as i64)).enumerate() {
            let block_is_empty = Cell::Empty == block[block_y][block_x];

            if block_is_empty {
                continue;
            }

            if x < 0 || y >= board.row_count as i64 {
                if block_is_empty {
                    continue;
                } else {
                    return false;
                }
            }

            let above_board = y < 0; // 위로 초과
            let next_board = x < 0 || x >= board.column_count.into(); // 옆으로 초과

            let y = y as usize;
            let x = x as usize;

            let cell = board.cells.get(y).map(|e| e.get(x)).flatten();

            match cell {
                Some(cell) => {
                    // 비어있는 영역에 시도는 유효
                    if let Cell::Empty = cell {
                        continue;
                    }
                    // 유효하지 않은 블럭 충돌
                    else {
                        return false;
                    }
                }
                None => {
                    if above_board {
                        if above_full {
                            return false;
                        }

                        if next_board {
                            return false;
                        }

                        continue;
                    }

                    // 블럭이 존재함에도 존재하지 않는 영역에 침범 시도
                    return false;
                }
            }
        }
    }

    true
}

pub fn valid_tspin(board: &Board, block: &BlockShape, point: Point, kick_try: usize) -> SpinType {
    let mut corner_fill_count: usize = 0; // if >=3 return true
    let rotation_count = block.rotation_count;
    let mut head_fill_count: usize = 0;

    for x in [point.x, point.x + 2] {
        for y in [point.y, point.y + 2] {
            //해당 위치가 보드 하좌우를 넘어서는 경우
            if x < 0 || x >= board.column_count.into() || y >= board.row_count as i64 {
                corner_fill_count += 1;
                continue;
            } else {
                if !board.cells[y as usize][x as usize].is_empty() {
                    corner_fill_count += 1;
                    match rotation_count {
                        0 => {
                            if y - point.y == 0 {
                                head_fill_count += 1;
                            }
                        }
                        1 => {
                            if x - point.x == 2 {
                                head_fill_count += 1;
                            }
                        }
                        2 => {
                            if y - point.y == 2 {
                                head_fill_count += 1;
                            }
                        }
                        3 => {
                            if x - point.x == 0 {
                                head_fill_count += 1;
                            }
                        }
                        _ => {}
                    }
                    continue;
                }
            }
        }
    }

    if corner_fill_count >= 3 {
        if head_fill_count == 2 || kick_try == 3 {
            return SpinType::TSpin;
        } else {
            return SpinType::Mini;
        }
    } else {
        return SpinType::None;
    }
}
