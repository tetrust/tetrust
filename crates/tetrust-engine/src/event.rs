//! 엔진에 들어가는 이벤트와 엔진이 내놓는 결과.

use crate::piece::PieceKind;

/// 게임 시계. 매치 시작부터 센 정수 밀리초.
pub type Millis = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Left,
    Right,
    SoftDrop,
    HardDrop,
    RotateCw,
    RotateCcw,
    Rotate180,
    Hold,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    Press(Action),
    Release(Action),
    /// 상대가 보낸 가비지가 **이 플레이어의 시계로** 도착한 시각에 기록된다.
    Garbage {
        lines: u32,
    },
}

/// 입력 로그 한 줄. `(time, seq)`가 엄격히 증가해야 한다.
///
/// 같은 밀리초 안의 이벤트는 `seq`로 순서를 정한다. 클라이언트가 브라우저에서 받은 순서대로
/// 매기고, 서버 리플레이도 같은 순서를 쓴다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Event {
    pub time: Millis,
    pub seq: u64,
    pub kind: EventKind,
}

impl Event {
    pub fn new(time: Millis, seq: u64, kind: EventKind) -> Self {
        Self { time, seq, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinKind {
    #[default]
    None,
    Mini,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopOutReason {
    /// 새 조각이 나올 자리가 막혔다.
    BlockOut,
    /// 조각이 전부 보이는 영역 위에서 고정됐다.
    LockOut,
    /// 가비지가 블록을 보드 위로 밀어냈다.
    GarbageOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LockResult {
    pub piece: PieceKind,
    pub lines: u32,
    pub spin: SpinKind,
    /// 연속으로 줄을 지운 횟수. 첫 삭제가 0.
    pub combo: Option<u32>,
    /// 어려운 삭제(테트리스, 스핀)가 연속된 횟수. 첫 번째가 0.
    pub back_to_back: Option<u32>,
    pub perfect_clear: bool,
    /// 상쇄 전 공격량.
    pub attack: u32,
}

/// 엔진이 내놓는 결과. 렌더러, 효과음, 네트워크 계층이 소비한다.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Locked {
        time: Millis,
        result: LockResult,
    },
    /// 상쇄하고 남은 공격. 네트워크 계층이 상대에게 보낸다.
    AttackSent {
        time: Millis,
        lines: u32,
    },
    GarbageApplied {
        time: Millis,
        lines: u32,
        hole: i32,
    },
    TopOut {
        time: Millis,
        reason: TopOutReason,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventError {
    /// `(time, seq)`가 직전 이벤트보다 크지 않다.
    OutOfOrder,
}
