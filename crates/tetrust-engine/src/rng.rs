//! 결정적 난수. 같은 시드면 어느 플랫폼에서든 같은 수열이 나온다.

use crate::piece::PieceKind;

/// SplitMix64.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// 0..n 범위의 수.
    pub fn below(&mut self, n: u64) -> u64 {
        self.next_u64() % n
    }
}

/// 7-bag 랜덤 생성기.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SevenBag {
    rng: Rng,
}

impl SevenBag {
    pub fn new(seed: u64) -> Self {
        Self {
            rng: Rng::new(seed),
        }
    }

    pub fn next_bag(&mut self) -> [PieceKind; 7] {
        let mut bag = PieceKind::ALL;
        for i in (1..bag.len()).rev() {
            let j = self.rng.below(i as u64 + 1) as usize;
            bag.swap(i, j);
        }
        bag
    }
}
