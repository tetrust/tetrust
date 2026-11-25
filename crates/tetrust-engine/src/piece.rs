//! 테트로미노 모양과 회전.
//!
//! 좌표계는 SRS 표준과 같은 y-up이다. 조각의 위치는 회전 박스의 왼쪽 아래 칸이고,
//! 각 셀은 박스 안의 (x, y) 오프셋으로 표현한다.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PieceKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl PieceKind {
    pub const ALL: [PieceKind; 7] = [
        PieceKind::I,
        PieceKind::J,
        PieceKind::L,
        PieceKind::O,
        PieceKind::S,
        PieceKind::T,
        PieceKind::Z,
    ];

    /// 회전 박스 한 변의 길이.
    pub fn box_size(self) -> i32 {
        match self {
            PieceKind::I => 4,
            PieceKind::O => 2,
            _ => 3,
        }
    }

    /// 스폰 상태(0)의 셀. 박스 안 y-up 좌표.
    fn spawn_cells(self) -> [(i32, i32); 4] {
        match self {
            PieceKind::I => [(0, 2), (1, 2), (2, 2), (3, 2)],
            PieceKind::J => [(0, 2), (0, 1), (1, 1), (2, 1)],
            PieceKind::L => [(2, 2), (0, 1), (1, 1), (2, 1)],
            PieceKind::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            PieceKind::S => [(1, 2), (2, 2), (0, 1), (1, 1)],
            PieceKind::T => [(1, 2), (0, 1), (1, 1), (2, 1)],
            PieceKind::Z => [(0, 2), (1, 2), (1, 1), (2, 1)],
        }
    }

    /// 주어진 회전 상태의 셀 오프셋.
    pub fn cells(self, rotation: Rotation) -> [(i32, i32); 4] {
        let n = self.box_size();
        let mut cells = self.spawn_cells();
        for _ in 0..rotation.0 {
            for c in cells.iter_mut() {
                // 박스 중심 기준 시계방향 90도 (y-up)
                *c = (c.1, n - 1 - c.0);
            }
        }
        cells
    }
}

/// 회전 상태. 0 = 스폰, 1 = R(시계), 2 = 180, 3 = L(반시계).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rotation(pub u8);

impl Rotation {
    pub fn cw(self) -> Self {
        Rotation((self.0 + 1) % 4)
    }

    pub fn ccw(self) -> Self {
        Rotation((self.0 + 3) % 4)
    }

    pub fn flip(self) -> Self {
        Rotation((self.0 + 2) % 4)
    }
}

/// 보드 위에서 움직이는 조각.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActivePiece {
    pub kind: PieceKind,
    pub rotation: Rotation,
    /// 회전 박스 왼쪽 아래 칸의 보드 좌표.
    pub x: i32,
    pub y: i32,
}

impl ActivePiece {
    /// 보드 좌표로 변환한 네 칸.
    pub fn cells(&self) -> [(i32, i32); 4] {
        self.kind
            .cells(self.rotation)
            .map(|(cx, cy)| (self.x + cx, self.y + cy))
    }

    pub fn shifted(&self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            ..*self
        }
    }
}
