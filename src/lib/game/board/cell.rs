use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty = "white",
    Ghost = "#d3d3d3",
    Garbage = "#424242",
    Red = "red",
    Green = "green",
    Blue = "blue",
    Purple = "purple",
    Cyan = "cyan",
    Orange = "orange",
    Yellow = "yellow",

    Suggested = "#B1BED5",
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

impl std::convert::TryFrom<i32> for Cell {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, ()> {
        match value {
            0 => Ok(Cell::Empty),
            1 => Ok(Cell::Red),
            2 => Ok(Cell::Green),
            3 => Ok(Cell::Blue),
            4 => Ok(Cell::Purple),
            5 => Ok(Cell::Cyan),
            6 => Ok(Cell::Orange),
            7 => Ok(Cell::Yellow),
            8 => Ok(Cell::Ghost),
            9 => Ok(Cell::Garbage),
            _ => Err(()),
        }
    }
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        self == &Self::Empty
    }

    pub fn into_code(&self) -> i32 {
        match self {
            Self::Empty => 0,
            Self::Red => 1,
            Self::Green => 2,
            Self::Blue => 3,
            Self::Purple => 4,
            Self::Cyan => 5,
            Self::Orange => 6,
            Self::Yellow => 7,
            Self::Ghost => 8,
            Self::Garbage => 9,
            _ => 0,
        }
    }

    pub fn to_color(&self) -> &str {
        match self {
            Self::Empty => "white",
            Self::Red => "#db1532",
            Self::Green => "#5bab08",
            Self::Blue => "#092fca",
            Self::Purple => "#6e00ff",
            Self::Cyan =>"#099dca",
            Self::Orange =>  "#e35c33",
            Self::Yellow =>  "#dc9f09",
            Self::Ghost => "#6d6d6d",
            Self::Garbage => "#424242",
            _ => "white",
        }
    }
}
