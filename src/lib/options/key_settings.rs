use web_sys::KeyboardEvent;

pub struct KeySettings {
    pub left_1: u32,
    pub left_2: u32,
    pub left_3: u32,
    pub left_4: u32,
    pub board_height: u32,
    pub column_count: u32,
    pub row_count: u32,
    pub bag_mode: BagType,
}

impl Default for GameOption {
    fn default() -> Self {
        Self {
            column_count: 10,
            row_count: 20,
            bag_mode: BagType::SevenBag,
            board_width: 300,
            board_height: 600,
        }
    }
}

