#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DasChargingStatus {
    Left,
    Right,
    None,
}

impl DasChargingStatus {
    pub fn set_left(&mut self) {
        *self = Self::Left;
    }

    pub fn is_left(&self) -> bool {
        *self == Self::Left
    }

    pub fn set_right(&mut self) {
        *self = Self::Right;
    }

    pub fn is_right(&self) -> bool {
        *self == Self::Right
    }

    pub fn set_none(&mut self) {
        *self = Self::None;
    }

    pub fn is_none(&self) -> bool {
        *self == Self::None
    }
}

impl Default for DasChargingStatus {
    fn default() -> Self {
        Self::None
    }
}
