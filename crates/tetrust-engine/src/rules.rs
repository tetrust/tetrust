//! 공격량 계산.

use crate::event::SpinKind;

const COMBO_TABLE: [u32; 13] = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4, 4, 4, 5];
pub const PERFECT_CLEAR_BONUS: u32 = 10;

/// 테트리스나 줄을 지운 스핀은 "어려운 삭제"로 B2B를 잇는다.
pub fn is_difficult(lines: u32, spin: SpinKind) -> bool {
    lines == 4 || (lines > 0 && spin != SpinKind::None)
}

pub fn base_attack(lines: u32, spin: SpinKind) -> u32 {
    match (spin, lines) {
        (_, 0) => 0,
        (SpinKind::Full, n) => 2 * n,
        (SpinKind::Mini, n) => n - 1,
        (SpinKind::None, 4) => 4,
        (SpinKind::None, n) => n - 1,
    }
}

pub fn attack(
    lines: u32,
    spin: SpinKind,
    combo: Option<u32>,
    back_to_back: Option<u32>,
    perfect_clear: bool,
) -> u32 {
    if lines == 0 {
        return 0;
    }
    let mut total = base_attack(lines, spin);
    if back_to_back.is_some_and(|b| b > 0) {
        total += 1;
    }
    if let Some(c) = combo {
        total += COMBO_TABLE[(c as usize).min(COMBO_TABLE.len() - 1)];
    }
    if perfect_clear {
        total += PERFECT_CLEAR_BONUS;
    }
    total
}
