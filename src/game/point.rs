#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn start_point(column_count: u32) -> Self {
        let y = 2;
        let x = column_count as i64 / 2 - 2;

        Self { x, y }
    }

    pub fn add_x(mut self, x: i64) -> Self {
        self.x = self.x + x;
        self
    }

    pub fn add_y(mut self, y: i64) -> Self {
        self.y = self.y + y;
        self
    }

    pub fn move_xy(mut self, x: i64, y: i64) -> Self {
        self.x = self.x + x;
        self.y = self.y + y;
        self
    }

}
