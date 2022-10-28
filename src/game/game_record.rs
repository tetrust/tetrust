#[derive(Debug, Clone, Default)]
pub struct GameRecord {
    pub score: u64,         // 점수
    pub line: u32,          // 지운 줄 개수
    pub quad: u32,          // 4줄 지우기 개수
    pub tspin_single: u32,  // T스핀 싱글 횟수
    pub tspin_double: u32,  // T스핀 더블 횟수
    pub tspin_triple: u32,  // T스핀 트리플 횟수
    pub perfect_clear: u32, // 퍼펙트 클리어 횟수
    pub max_combo: u32,     // 최대 콤보
    pub back_to_back: u32,  // 최대 백투백
}
