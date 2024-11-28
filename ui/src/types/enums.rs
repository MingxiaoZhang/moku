use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoardSize {
    Small = 9,
    Medium = 13,
    Large = 19,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Stone {
    Empty,
    Black,
    White,
}

impl Stone {
    pub fn to_class(&self) -> &str {
        match self {
            Stone::White => "bg-[#FAF0E6] outline outline-1 outline-black",
            Stone::Black => "bg-[#352F44] outline outline-1 outline-black",
            Stone::Empty => "",
        }
    }
}