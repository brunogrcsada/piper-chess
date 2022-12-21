use super::pieces::GamePiece;
use super::players::Player;

#[derive(Copy, Clone, PartialEq)]
pub enum Cells {
    Missing,
    Piece(GamePiece, Player),
}

impl Cells {
    /* Convert piece to string value.

    This allows for images in the front-end to be matched
    to each piece.
    */
    pub fn display(&self) -> &str {
        match *self {
            // Return empty string if no piece is available
            Cells::Missing => "",

            // Black Pawn
            Cells::Piece(GamePiece::Pawn, Player::Black) => "b_pawn",
            // Black Rook
            Cells::Piece(GamePiece::Rook, Player::Black) => "b_rook",
            // Black Knight
            Cells::Piece(GamePiece::Knight, Player::Black) => "b_knight",
            // Black Bishop
            Cells::Piece(GamePiece::Bishop, Player::Black) => "b_bishop",
            // Black Queen
            Cells::Piece(GamePiece::Queen, Player::Black) => "b_queen",
            // Black King
            Cells::Piece(GamePiece::King, Player::Black) => "b_king",

            // White Pawn
            Cells::Piece(GamePiece::Pawn, Player::White) => "w_pawn",
            // White Rook
            Cells::Piece(GamePiece::Rook, Player::White) => "w_rook",
            // White Knight
            Cells::Piece(GamePiece::Knight, Player::White) => "w_knight",
            // White Bishop
            Cells::Piece(GamePiece::Bishop, Player::White) => "w_bishop",
            // White Queen
            Cells::Piece(GamePiece::Queen, Player::White) => "w_queen",
            // White King
            Cells::Piece(GamePiece::King, Player::White) => "w_king",
        }
    }

    pub fn player(&self) -> Option<Player> {
        match *self {
            Cells::Piece(_, c) => Some(c),
            Cells::Missing => None,
        }
    }
}
