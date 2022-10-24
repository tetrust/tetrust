use crate::game::bag::BagType;

pub struct GameOption {
    pub board_width: u32,
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
