#[derive(Debug, Clone, Default)]
pub struct GameRecord {
    pub score: u64,         // 점수
    pub line_clear_count: u32,          // 지운 줄 개수
    pub quad_count: u32,          // 4줄 지우기 개수
    pub tspin_single_count: u32,  // T스핀 싱글 횟수
    pub tspin_double_count: u32,  // T스핀 더블 횟수
    pub tspin_triple_count: u32,  // T스핀 트리플 횟수
    pub perfect_clear_count: u32, // 퍼펙트 클리어 횟수
    pub max_combo_count: u32,     // 최대 콤보
    pub max_back_to_back_count: u32,  // 최대 백투백
}
