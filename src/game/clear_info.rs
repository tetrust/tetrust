use super::spin_type::SpinType;

#[derive(Debug, Clone)]
pub struct ClearInfo {
    pub line: u8,
    pub is_perfect: bool,
    pub spin: SpinType,
}
