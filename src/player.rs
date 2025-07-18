use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Red,
    Yellow,
}

impl Player {
    pub fn opposite(self) -> Self {
        match self {
            Player::Red => Player::Yellow,
            Player::Yellow => Player::Red,
        }
    }
    
    pub fn to_string(self) -> &'static str {
        match self {
            Player::Red => "Red",
            Player::Yellow => "Yellow",
        }
    }
} 