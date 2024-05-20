use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Deserialize, Serialize, PartialEq, Copy, Clone)]
pub enum ClientId {
    GroundFloor,
    FirstFloor,
    SecondFloor,
}

impl ClientId {
    pub fn from_str(client_id: &str) -> Option<Self> {
        match client_id {
            "ground_floor" => Some(Self::GroundFloor),
            "first_floor" => Some(Self::FirstFloor),
            "second_floor" => Some(Self::SecondFloor),
            _ => None,
        }
    }

    pub fn to_index(&self) -> i32 {
        match self {
            Self::GroundFloor => 0,
            Self::FirstFloor => 1,
            Self::SecondFloor => 2,
        }
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::GroundFloor => write!(f, "ground_floor"),
            Self::FirstFloor => write!(f, "first_floor"),
            Self::SecondFloor => write!(f, "second_floor"),
        }
    }
}
