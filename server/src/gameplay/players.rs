#[derive(Copy, Clone, PartialEq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn other(&self) -> Player {
        match *self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }

    pub fn display(&self) -> &str {
        match *self {
            Player::White => "white",
            Player::Black => "black",
        }
    }
}
