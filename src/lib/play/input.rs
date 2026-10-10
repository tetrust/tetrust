//! 키보드 이벤트를 엔진 이벤트로 바꾸는 어댑터.

use tetrust_engine::{Action, Millis};

/// 키 배치. `KeyboardEvent.code`(물리 키 위치) 기준이라 자판 배열과 상관없다.
pub fn action_for(code: &str) -> Option<Action> {
    Some(match code {
        "ArrowLeft" => Action::Left,
        "ArrowRight" => Action::Right,
        "ArrowDown" => Action::SoftDrop,
        "Space" => Action::HardDrop,
        "ArrowUp" | "KeyX" => Action::RotateCw,
        "KeyZ" | "ControlLeft" => Action::RotateCcw,
        "KeyA" => Action::Rotate180,
        "ShiftLeft" | "ShiftRight" | "KeyC" => Action::Hold,
        _ => return None,
    })
}

/// 브라우저 타임스탬프(ms, `performance.now()`와 같은 기준)를 게임 시계로 바꾼다.
///
/// 렌더 루프가 시계를 이미 진행시킨 뒤에 그보다 이른 타임스탬프의 입력이 도착할 수 있다.
/// 엔진은 과거 시각의 입력을 거부하므로, 그런 입력은 현재 게임 시계로 당겨 기록한다.
/// 기록된 시각이 곧 입력 로그의 시각이라 리플레이 결과는 그대로 같다.
pub fn game_time(timestamp: f64, start: f64, clock: Millis) -> Millis {
    let elapsed = (timestamp - start).max(0.0).floor() as Millis;
    elapsed.max(clock)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_and_clamps_timestamps() {
        assert_eq!(game_time(1500.7, 1000.0, 0), 500);
        assert_eq!(game_time(900.0, 1000.0, 0), 0, "before start");
        assert_eq!(game_time(1200.0, 1000.0, 350), 350, "late input is clamped");
    }

    #[test]
    fn maps_default_keys() {
        assert_eq!(action_for("ArrowLeft"), Some(Action::Left));
        assert_eq!(action_for("KeyZ"), Some(Action::RotateCcw));
        assert_eq!(action_for("KeyQ"), None);
    }
}
