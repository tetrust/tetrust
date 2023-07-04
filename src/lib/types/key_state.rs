#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyState {
    pub left: bool,
    pub right: bool,
    pub down: bool,
}

impl KeyState {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            down: false,
        }
    }

    pub fn set_left(&mut self, value: bool) {
        self.left = value;
    }

    pub fn set_right(&mut self, value: bool) {
        self.right = value;
    }

    pub fn set_down(&mut self, value: bool) {
        self.down = value;
    }
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
