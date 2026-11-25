use crate::event::Millis;

/// 핸들링과 룰 설정. 모든 시간은 게임 시계 밀리초다.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// 중력: 한 칸 떨어지는 간격. 1 이상.
    pub gravity: Millis,
    /// 소프트 드롭 중 한 칸 간격. 0이면 바닥까지 즉시.
    pub soft_drop: Millis,
    /// 바닥에 닿은 뒤 고정까지의 시간.
    pub lock_delay: Millis,
    /// 바닥에서 이동·회전으로 락 딜레이를 다시 채울 수 있는 횟수.
    /// 조각이 더 낮은 줄에 도달하면 횟수가 초기화된다.
    pub lock_reset_limit: u32,
    /// 키를 누른 뒤 자동 이동이 시작되기까지의 시간.
    pub das: Millis,
    /// 자동 이동 반복 간격. 0이면 벽까지 즉시.
    pub arr: Millis,
    /// 미리보기 개수.
    pub next_previews: usize,
    /// 가비지가 도착한 뒤 보드에 들어갈 수 있게 될 때까지의 시간. 그 전에는 상쇄만 된다.
    pub garbage_delay: Millis,
    /// 한 번의 고정에서 들어올 수 있는 가비지 최대 줄 수.
    pub garbage_cap: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            gravity: 1000,
            soft_drop: 50,
            lock_delay: 500,
            lock_reset_limit: 15,
            das: 167,
            arr: 33,
            next_previews: 5,
            garbage_delay: 0,
            garbage_cap: 8,
        }
    }
}
