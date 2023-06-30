use crate::lib::game::SpinType;

pub fn calculate_score(
    line: u8,
    is_perfect: bool,
    combo: Option<u32>,
    spin_type: SpinType,
    back2back: Option<u32>,
) -> u64 {
    let mut score = 0;

    if is_perfect {
        score += 1000;
    }

    let back2back_bonus = match back2back {
        Some(back2back) => back2back * 100,
        None => 0,
    } as u64;

    match spin_type {
        SpinType::None => match line {
            1 => score += 10,
            2 => score += 20,
            3 => score += 30,
            4 => {
                score += 100;
            }
            _ => {}
        },
        SpinType::Spin => match line {
            1 => score += 50 + back2back_bonus,
            2 => score += 100 + back2back_bonus,
            _ => {}
        },
        SpinType::TSpin => match line {
            1 => score += 500 + back2back_bonus,
            2 => score += 1000 + back2back_bonus,
            3 => score += 2000 + back2back_bonus,
            _ => {}
        },
        SpinType::Mini => match line {
            1 => score += 50 + back2back_bonus,
            2 => score += 100 + back2back_bonus,
            _ => {}
        },
    }

    if let Some(combo) = combo {
        score += combo as u64 * 10;
    }

    score
}
