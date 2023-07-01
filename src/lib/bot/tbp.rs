use serde::{Deserialize, Serialize};

use crate::lib::game::Block;

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPInfo {
    pub name: String,
    pub version: String,
    pub author: String,
    pub features: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub enum TBPRandomizerType {
    #[serde(rename = "uniform")]
    Uniform,
    #[default]
    #[serde(rename = "seven_bag")]
    SevenBag,
    #[serde(rename = "general_bag")]
    GeneralBag,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPRules {
    pub randomizer: TBPRandomizerType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TBPRandomizerState {
    #[serde(rename = "seven_bag")]
    SevenBag(Vec<Block>),
    //  #[serde(rename = "general_bag")]
    //  GeneralBag(),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPStart {
    pub hold: Option<Block>,
    pub queue: Vec<Block>,
    pub combo: usize,
    pub back_to_back: bool,
    pub board: Vec<Vec<Option<Block>>>,
}

impl Default for TBPStart {
    fn default() -> Self {
        Self {
            hold: None,
            queue: vec![],
            combo: 0,
            back_to_back: false,
            board: vec![vec![None; 10]; 40],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TBPOrientation {
    #[serde(rename = "north")]
    North,
    #[serde(rename = "east")]
    East,
    #[serde(rename = "south")]
    South,
    #[serde(rename = "west")]
    West,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPLocation {
    pub orientation: TBPOrientation,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TBPSpin {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "mini")]
    Mini,
    #[serde(rename = "full")]
    Full,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPMove {
    pub location: TBPLocation,
    pub spin: TBPSpin,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPSuggestion {
    pub moves: Vec<TBPMove>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPPlay {
    #[serde(rename = "move")]
    pub move_: TBPMove,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TBPNewPiece {
    piece: Block,
}

macro_rules! EmptyMessage {
    ($name:ident) => {
        #[derive(Debug, Deserialize, Serialize)]
        pub struct $name {}
    };
}

EmptyMessage!(TBPReady);
EmptyMessage!(TBPSuggest);
EmptyMessage!(TBPStop);
EmptyMessage!(TBPQuit);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    Info(TBPInfo),
    Rules(TBPRules),
    Ready(TBPReady),
    Start(TBPStart),
    Suggest(TBPSuggest),
    Suggestion(TBPSuggestion),
    Play(TBPPlay),
    NewPiece(TBPNewPiece),
    Stop(TBPStop),
    Quit(TBPQuit),
}
