impl GamePiece {
    pub fn parse(string: &str) -> Option<GamePiece> {
        match string {
            "bishop" => Some(GamePiece::Bishop),
            "pawn" => Some(GamePiece::Pawn),
            "rook" => Some(GamePiece::Rook),
            "queen" => Some(GamePiece::Queen),
            "king" => Some(GamePiece::King),
            "knight" => Some(GamePiece::Knight),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum GamePiece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
