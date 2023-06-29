#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DasChargingStatus {
    pub left: bool,
    pub right: bool,
}

impl Default for DasChargingStatus {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
        }
    }
}
