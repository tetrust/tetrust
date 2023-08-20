// 키보드 제어 이벤트
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    LeftMove,
    LeftMoveStop,
    RightMove,
    RightMoveStop,
    LeftRotate,
    RightRotate,
    SoftDrop,
    HardDrop,
    DoubleRotate,
    Hold,
}
