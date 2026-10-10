//! 엔진 결과(Outcome)를 모아 화면에 보여 줄 통계로 만든다.

use tetrust_engine::{LockResult, Millis, Outcome, SpinKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    #[default]
    Normal,
    /// 40줄을 지우면 끝난다.
    Sprint,
}

pub const SPRINT_LINES: u32 = 40;

#[derive(Debug, Clone, Default)]
pub struct Stats {
    pub pieces: u32,
    pub lines: u32,
    pub attack: u32,
    pub quads: u32,
    pub perfect_clears: u32,
    pub combo: Option<u32>,
    pub back_to_back: Option<u32>,
    /// 마지막으로 보여 줄 만한 삭제 이름.
    pub message: String,
    /// 스프린트 완료 시각.
    pub finished_at: Option<Millis>,
    pub topped_out: bool,
}

impl Stats {
    pub fn apply(&mut self, outcome: &Outcome, mode: Mode) {
        match *outcome {
            Outcome::Locked { time, result } => {
                self.pieces += 1;
                self.lines += result.lines;
                self.attack += result.attack;
                self.combo = result.combo;
                self.back_to_back = result.back_to_back;
                if result.lines == 4 {
                    self.quads += 1;
                }
                if result.perfect_clear {
                    self.perfect_clears += 1;
                }
                if let Some(name) = clear_name(&result) {
                    self.message = name;
                }
                if mode == Mode::Sprint && self.lines >= SPRINT_LINES && self.finished_at.is_none()
                {
                    self.finished_at = Some(time);
                }
            }
            Outcome::TopOut { .. } => self.topped_out = true,
            Outcome::AttackSent { .. } | Outcome::GarbageApplied { .. } => {}
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished_at.is_some() || self.topped_out
    }

    pub fn pieces_per_second(&self, clock: Millis) -> f64 {
        if clock == 0 {
            0.0
        } else {
            self.pieces as f64 * 1000.0 / clock as f64
        }
    }
}

fn clear_name(result: &LockResult) -> Option<String> {
    let lines = match result.lines {
        1 => "Single",
        2 => "Double",
        3 => "Triple",
        4 => "Quad",
        _ => "",
    };
    let name = match result.spin {
        SpinKind::Full => format!("T-Spin {lines}"),
        SpinKind::Mini => format!("T-Spin Mini {lines}"),
        SpinKind::None if result.lines == 0 => return None,
        SpinKind::None => lines.to_string(),
    };
    let name = if result.perfect_clear {
        format!("Perfect Clear {name}")
    } else {
        name
    };
    Some(name.trim().to_string())
}

/// 초 단위, 소수점 둘째 자리.
pub fn format_time(ms: Millis) -> String {
    format!("{}.{:02}", ms / 1000, ms % 1000 / 10)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tetrust_engine::PieceKind;

    fn lock(time: Millis, lines: u32, spin: SpinKind) -> Outcome {
        Outcome::Locked {
            time,
            result: LockResult {
                piece: PieceKind::T,
                lines,
                spin,
                combo: Some(0),
                back_to_back: None,
                perfect_clear: false,
                attack: 0,
            },
        }
    }

    #[test]
    fn sprint_finishes_at_forty_lines() {
        let mut stats = Stats::default();
        for i in 0..9 {
            stats.apply(&lock(i * 1000, 4, SpinKind::None), Mode::Sprint);
        }
        assert_eq!(stats.finished_at, None);
        stats.apply(&lock(9500, 4, SpinKind::None), Mode::Sprint);
        assert_eq!(stats.finished_at, Some(9500));
        assert_eq!(stats.quads, 10);
    }

    #[test]
    fn names_spins() {
        let mut stats = Stats::default();
        stats.apply(&lock(0, 2, SpinKind::Full), Mode::Normal);
        assert_eq!(stats.message, "T-Spin Double");
        stats.apply(&lock(1, 0, SpinKind::Mini), Mode::Normal);
        assert_eq!(stats.message, "T-Spin Mini");
    }

    #[test]
    fn formats_time() {
        assert_eq!(format_time(61_234), "61.23");
        assert_eq!(format_time(5), "0.00");
    }
}
