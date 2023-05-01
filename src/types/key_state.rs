#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyState {
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

impl Default for KeyState {
    fn default() -> Self {
        Self {
            left: false,
            right: false,
            down: false,
        }
    }
}
