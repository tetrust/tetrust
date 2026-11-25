//! 캔버스 렌더러.
//!
//! 컨텍스트는 한 번만 얻어 두고, 프레임마다 같은 색 칸을 모아 `fill_style`을 색당 한 번만
//! 바꾼다. 셀마다 JS 경계를 여러 번 넘던 예전 방식보다 호출 수가 크게 줄어든다.

use tetrust_engine::{Cell, Game, PieceKind, Rotation, VISIBLE_HEIGHT, WIDTH};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement};

/// 보이는 20줄 위로 스폰 영역 2줄을 같이 그린다.
pub const DRAWN_ROWS: i32 = VISIBLE_HEIGHT + 2;
pub const CELL: f64 = 30.0;
const PREVIEW_CELL: f64 = 24.0;

const BACKGROUND: &str = "#080808";
const GRID: &str = "#1c1c1c";
const BUFFER_BACKGROUND: &str = "#000000";
const GAUGE_BACKGROUND: &str = "#2a2a2a";
const GAUGE_FILL: &str = "#ff4040";

/// 색 버킷 순서. `PieceKind` 7개 + 가비지.
const COLORS: [&str; 8] = [
    "#31c7ef", // I
    "#5a65ad", // J
    "#ef7921", // L
    "#f7d308", // O
    "#42b642", // S
    "#ad4d9c", // T
    "#ef2029", // Z
    "#6b6b6b", // garbage
];

fn bucket(kind: PieceKind) -> usize {
    match kind {
        PieceKind::I => 0,
        PieceKind::J => 1,
        PieceKind::L => 2,
        PieceKind::O => 3,
        PieceKind::S => 4,
        PieceKind::T => 5,
        PieceKind::Z => 6,
    }
}

const GARBAGE_BUCKET: usize = 7;

fn context(
    document: &Document,
    id: &str,
    low_latency: bool,
) -> Result<CanvasRenderingContext2d, JsValue> {
    let canvas: HtmlCanvasElement = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("missing #{id}")))?
        .dyn_into()?;
    let options = js_sys::Object::new();
    if low_latency {
        // 합성 단계를 건너뛰어 화면 표시 지연을 줄인다(지원하는 브라우저에서만).
        js_sys::Reflect::set(&options, &"desynchronized".into(), &true.into())?;
    }
    canvas
        .get_context_with_context_options("2d", &options)?
        .ok_or_else(|| JsValue::from_str("2d context unavailable"))?
        .dyn_into()
        .map_err(JsValue::from)
}

/// 색별로 모은 칸들. 프레임마다 재사용해서 할당을 피한다.
#[derive(Default)]
struct Batch {
    rects: [Vec<(f64, f64)>; 8],
}

impl Batch {
    fn clear(&mut self) {
        self.rects.iter_mut().for_each(Vec::clear);
    }

    fn flush(&self, ctx: &CanvasRenderingContext2d, size: f64) {
        for (color, rects) in COLORS.iter().zip(&self.rects) {
            if rects.is_empty() {
                continue;
            }
            ctx.set_fill_style_str(color);
            for &(x, y) in rects {
                ctx.fill_rect(x + 1.0, y + 1.0, size - 2.0, size - 2.0);
            }
        }
    }
}

pub struct Renderer {
    board: CanvasRenderingContext2d,
    hold: CanvasRenderingContext2d,
    next: CanvasRenderingContext2d,
    gauge: CanvasRenderingContext2d,
    batch: Batch,
}

impl Renderer {
    pub fn new(document: &Document) -> Result<Self, JsValue> {
        Ok(Self {
            board: context(document, "game-canvas", true)?,
            hold: context(document, "hold-canvas", false)?,
            next: context(document, "next-canvas", false)?,
            gauge: context(document, "garbage-gauge-canvas", false)?,
            batch: Batch::default(),
        })
    }

    /// 게임 시작 전 빈 화면.
    pub fn draw_idle(&mut self) {
        self.draw_board_background();
        self.draw_hold(None);
        self.next.set_fill_style_str(BACKGROUND);
        self.next.fill_rect(0.0, 0.0, 120.0, 520.0);
        self.draw_gauge(0);
    }

    pub fn draw(&mut self, game: &Game) {
        self.draw_board(game);
        self.draw_hold(game.hold_piece());
        self.draw_next(game);
        self.draw_gauge(game.pending_garbage());
    }

    /// 보드 좌표(y-up) → 캔버스 좌표.
    fn to_screen(x: i32, y: i32) -> (f64, f64) {
        (x as f64 * CELL, (DRAWN_ROWS - 1 - y) as f64 * CELL)
    }

    fn draw_board_background(&self) {
        let ctx = &self.board;
        let width = WIDTH as f64 * CELL;
        let buffer = (DRAWN_ROWS - VISIBLE_HEIGHT) as f64 * CELL;
        let height = DRAWN_ROWS as f64 * CELL;

        ctx.set_fill_style_str(BUFFER_BACKGROUND);
        ctx.fill_rect(0.0, 0.0, width, buffer);
        ctx.set_fill_style_str(BACKGROUND);
        ctx.fill_rect(0.0, buffer, width, height - buffer);

        // 격자는 경로 하나로 한 번에 그린다.
        ctx.begin_path();
        for x in 1..WIDTH {
            ctx.move_to(x as f64 * CELL, buffer);
            ctx.line_to(x as f64 * CELL, height);
        }
        for y in 1..VISIBLE_HEIGHT {
            ctx.move_to(0.0, buffer + y as f64 * CELL);
            ctx.line_to(width, buffer + y as f64 * CELL);
        }
        ctx.set_stroke_style_str(GRID);
        ctx.stroke();
    }

    fn draw_board(&mut self, game: &Game) {
        self.draw_board_background();
        let ctx = &self.board;

        self.batch.clear();
        for (y, row) in game
            .board()
            .rows()
            .iter()
            .enumerate()
            .take(DRAWN_ROWS as usize)
        {
            for (x, cell) in row.iter().enumerate() {
                let b = match cell {
                    Cell::Empty => continue,
                    Cell::Piece(kind) => bucket(*kind),
                    Cell::Garbage => GARBAGE_BUCKET,
                };
                self.batch.rects[b].push(Self::to_screen(x as i32, y as i32));
            }
        }
        if let Some(piece) = game.active() {
            for (x, y) in piece.cells() {
                if y < DRAWN_ROWS {
                    self.batch.rects[bucket(piece.kind)].push(Self::to_screen(x, y));
                }
            }
        }
        self.batch.flush(ctx, CELL);

        if let (Some(ghost), Some(piece)) = (game.ghost(), game.active()) {
            if ghost.y != piece.y {
                self.batch.clear();
                for (x, y) in ghost.cells() {
                    if y < DRAWN_ROWS {
                        self.batch.rects[bucket(ghost.kind)].push(Self::to_screen(x, y));
                    }
                }
                ctx.set_global_alpha(0.3);
                self.batch.flush(ctx, CELL);
                ctx.set_global_alpha(1.0);
            }
        }
    }

    /// 미리보기 칸 하나(가로 120px)에 조각을 가운데 정렬해 그린다.
    fn draw_preview(
        batch: &mut Batch,
        kind: PieceKind,
        slot_top: f64,
        slot_width: f64,
        slot_height: f64,
    ) {
        let cells = kind.cells(Rotation(0));
        let min_x = cells.iter().map(|c| c.0).min().unwrap_or(0);
        let max_x = cells.iter().map(|c| c.0).max().unwrap_or(0);
        let min_y = cells.iter().map(|c| c.1).min().unwrap_or(0);
        let max_y = cells.iter().map(|c| c.1).max().unwrap_or(0);
        let w = (max_x - min_x + 1) as f64 * PREVIEW_CELL;
        let h = (max_y - min_y + 1) as f64 * PREVIEW_CELL;
        let left = (slot_width - w) / 2.0;
        let top = slot_top + (slot_height - h) / 2.0;
        for (x, y) in cells {
            batch.rects[bucket(kind)].push((
                left + (x - min_x) as f64 * PREVIEW_CELL,
                top + (max_y - y) as f64 * PREVIEW_CELL,
            ));
        }
    }

    fn draw_hold(&mut self, hold: Option<PieceKind>) {
        let ctx = &self.hold;
        ctx.set_fill_style_str(BACKGROUND);
        ctx.fill_rect(0.0, 0.0, 120.0, 120.0);
        if let Some(kind) = hold {
            self.batch.clear();
            Self::draw_preview(&mut self.batch, kind, 0.0, 120.0, 120.0);
            self.batch.flush(ctx, PREVIEW_CELL);
        }
    }

    fn draw_next(&mut self, game: &Game) {
        let ctx = &self.next;
        ctx.set_fill_style_str(BACKGROUND);
        ctx.fill_rect(0.0, 0.0, 120.0, 520.0);
        self.batch.clear();
        for (i, kind) in game.next_pieces().enumerate() {
            Self::draw_preview(&mut self.batch, kind, i as f64 * 104.0, 120.0, 104.0);
        }
        self.batch.flush(ctx, PREVIEW_CELL);
    }

    fn draw_gauge(&mut self, pending: u32) {
        let ctx = &self.gauge;
        let height = DRAWN_ROWS as f64 * CELL;
        ctx.set_fill_style_str(GAUGE_BACKGROUND);
        ctx.fill_rect(0.0, 0.0, 30.0, height);
        let bar = (pending as f64 * CELL).min(VISIBLE_HEIGHT as f64 * CELL);
        ctx.set_fill_style_str(GAUGE_FILL);
        ctx.fill_rect(0.0, height - bar, 30.0, bar);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tetrust_engine::ActivePiece;

    #[test]
    fn bottom_left_cell_maps_to_bottom_left_of_canvas() {
        assert_eq!(Renderer::to_screen(0, 0), (0.0, 21.0 * CELL));
        assert_eq!(Renderer::to_screen(9, 21), (9.0 * CELL, 0.0));
    }

    #[test]
    fn spawned_piece_is_inside_the_drawn_area() {
        let piece = ActivePiece {
            kind: PieceKind::T,
            rotation: Rotation(0),
            x: 3,
            y: VISIBLE_HEIGHT - 1,
        };
        assert!(piece
            .cells()
            .iter()
            .all(|&(x, y)| Renderer::to_screen(x, y).1 >= 0.0));
    }
}
