use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Copy, Clone)]
pub enum ClientId {
    GroundFloor,
    FirstFloor,
    SecondFloor,
    App,
}

impl ClientId {
    pub fn from_str(client_id: &str) -> Option<Self> {
        match client_id {
            "ground_floor" => Some(Self::GroundFloor),
            "first_floor" => Some(Self::FirstFloor),
            "second_floor" => Some(Self::SecondFloor),
            "app" => Some(Self::App),
            _ => None,
        }
    }

    pub fn to_index(&self) -> i32 {
        match self {
            Self::GroundFloor => 0,
            Self::FirstFloor => 1,
            Self::SecondFloor => 2,
            Self::App => 3,
        }
    }

    pub fn from_index(index: i32) -> Option<Self> {
        match index {
            0 => Some(Self::GroundFloor),
            1 => Some(Self::FirstFloor),
            2 => Some(Self::SecondFloor),
            3 => Some(Self::App),
            _ => None,
        }
    }

    pub fn iter_floors() -> impl Iterator<Item = Self> {
        [Self::GroundFloor, Self::FirstFloor, Self::SecondFloor]
            .iter()
            .copied()
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::GroundFloor => write!(f, "ground_floor"),
            Self::FirstFloor => write!(f, "first_floor"),
            Self::SecondFloor => write!(f, "second_floor"),
            Self::App => write!(f, "app"),
        }
    }
}
