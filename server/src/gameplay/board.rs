use super::cells::Cells;
use super::moves::{Location, Move, MoveType};
use super::pieces::GamePiece;
use super::players::Player;
use std::collections::HashMap;
use std::ops::Range;

#[derive(Clone)]
pub struct ChessBoard {
    // Basic array definition
    board: [[Cells; 8]; 8],
    // Hold all moves in a vector
    moves: Vec<Move>,
}

impl ChessBoard {
    // Define base game world
    pub const fn new() -> ChessBoard {
        ChessBoard {
            moves: vec![],
            board: [
                // First row, base white pieces
                [
                    // White Rook
                    Cells::Piece(GamePiece::Rook, Player::White),
                    // White Knight
                    Cells::Piece(GamePiece::Knight, Player::White),
                    // White Bishop
                    Cells::Piece(GamePiece::Bishop, Player::White),
                    // White Queen
                    Cells::Piece(GamePiece::Queen, Player::White),
                    // White King
                    Cells::Piece(GamePiece::King, Player::White),
                    // Whtite Bishop
                    Cells::Piece(GamePiece::Bishop, Player::White),
                    // White Knight
                    Cells::Piece(GamePiece::Knight, Player::White),
                    // Whiet Rook
                    Cells::Piece(GamePiece::Rook, Player::White),
                ],
                // Second row, all white pawns
                [Cells::Piece(GamePiece::Pawn, Player::White); 8],
                // Empty spaces
                [Cells::Missing; 8],
                [Cells::Missing; 8],
                [Cells::Missing; 8],
                [Cells::Missing; 8],
                // Second to last row, all black pawns
                [Cells::Piece(GamePiece::Pawn, Player::Black); 8],
                // Final row, base black pieces
                [
                    Cells::Piece(GamePiece::Rook, Player::Black),
                    Cells::Piece(GamePiece::Knight, Player::Black),
                    Cells::Piece(GamePiece::Bishop, Player::Black),
                    Cells::Piece(GamePiece::Queen, Player::Black),
                    Cells::Piece(GamePiece::King, Player::Black),
                    Cells::Piece(GamePiece::Bishop, Player::Black),
                    Cells::Piece(GamePiece::Knight, Player::Black),
                    Cells::Piece(GamePiece::Rook, Player::Black),
                ],
            ],
        }
    }

    // Apply shift to chess board
    fn apply(&self, shift: Move) -> ChessBoard {
        let mut updated = self.clone();

        // Define originating co-ordinates
        let from_x: usize = shift.origin.x;
        let from_y: usize = shift.origin.y;

        // Define destination co-ordinates
        let to_x: usize = shift.destination.x;
        let to_y: usize = shift.destination.y;

        // Update destination cell
        updated.board[to_y][to_x] = self.at(shift.origin);

        // Make previous cell empty
        updated.board[from_y][from_x] = Cells::Missing;

        // Push move to board
        updated.moves.push(shift);
        updated
    }

    fn set(&self, co_ordinate: Location, cells: Cells) -> ChessBoard {
        // Clone existing board
        let mut updated = self.clone();

        // Update cell co-ordinates
        updated.board[co_ordinate.y][co_ordinate.x] = cells;
        updated
    }

    fn at(&self, co_ordinate: Location) -> Cells {
        self.board[co_ordinate.y][co_ordinate.x]
    }

    // Use internal methods in self to validate all moves for each piece
    fn validate_move(&self, piece: GamePiece, shift: Move, player: Player) -> bool {
        match piece {
            // Validate Queen (combine rook and bishop)
            GamePiece::Queen => self.validate_rook(shift) || self.validate_bishop(shift),

            // Validae Pawn
            GamePiece::Pawn => self.validate_pawn(shift, player, false),

            // Validate Knight
            GamePiece::Knight => self.validate_knight(shift),

            // Validate King
            GamePiece::King => self.validate_king(shift),

            // Validate Bishop
            GamePiece::Bishop => self.validate_bishop(shift),

            // Validate Rook
            GamePiece::Rook => self.validate_rook(shift),
        }
    }

    // Full game status to JSON
    pub fn output(&self) -> HashMap<String, String> {
        // All possible letters in the board
        let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

        // Structure for the game tiles
        let mut game: HashMap<String, String> = HashMap::new();

        // Iterate through every row
        for (row, num) in self.board.iter().zip(0u8..8).rev() {
            // Iterate through every tile in each row
            for (index, tile) in row.iter().enumerate() {
                // Full piece name, matching image files in front-end
                let piece = tile.display().to_string();

                // Create a board reference based on column letter and tile index
                let square = letters[index as usize].to_string();
                let reference = [square, (num + 1).to_string()].join("");

                // Add cell to hash map
                game.insert(reference, piece);
            }
        }

        return game;
    }

    // Handle different moves in a chess game
    /* Not all moves have been implemented for an extended game
    Enpassant and castling were left out in this MVP version */
    pub fn validate(&self, types: Option<MoveType>, player: Player) -> bool {
        match types {
            Some(shift_type) => {
                (match shift_type {
                    // All basic moves are handled in React front-end
                    MoveType::Normal(shift) => self.basic(shift, player),

                    // Piece reaches the end of the board and can be swapped with another
                    MoveType::Promotion(shift, piece) => self.promotion(shift, player, piece),
                }) && !self.apply_move(shift_type).check(player)
            }
            None => false,
        }
    }

    // General implementation for each move type
    pub fn apply_move(&self, moves: MoveType) -> ChessBoard {
        match moves {
            // Handle normal piece moves
            MoveType::Normal(shift) => self.apply(shift),
            // Handle pawn promotions
            MoveType::Promotion(shift, piece) => self.apply(shift).set(
                shift.destination,
                Cells::Piece(piece, self.at(shift.origin).player().unwrap()),
            ),
        }
    }

    // Validity for Rook piece
    fn validate_rook(&self, shift: Move) -> bool {
        // Define shifts from co-ordinates
        let from_x: usize = shift.origin.x;
        let from_y: usize = shift.origin.y;

        // Define shifts to co-ordinates
        let to_x: usize = shift.destination.x;
        let to_y: usize = shift.destination.y;

        // Is originating x co-ordinate equal to destination
        from_x == to_x
            // Is originating y co-ordinate greater than destination
            && if from_y > to_y {
                to_y + 1..from_y
            } else {
                from_y + 1..to_y
            }
            // Do all values match predicate (empty cell value)
            .all(|y_value: usize| self.board[y_value][to_x] == Cells::Missing)
            || from_y == to_y
                && if from_x > to_x {
                    to_x + 1..from_x
                } else {
                    from_x + 1..to_x
                }
                .all(|x: usize| self.board[shift.destination.y][x] == Cells::Missing)
    }

    fn validate_knight(&self, shift: Move) -> bool {
        // Define originating co-ordinates
        let from_x: usize = shift.origin.x;
        let from_y: usize = shift.origin.y;

        // Define destination co-ordinates
        let to_x: usize = shift.destination.x;
        let to_y: usize = shift.destination.y;

        // Define shifts for x and y axis
        let shift_x: i8 = (from_x as i8 - to_x as i8).abs();
        let shift_y: i8 = (from_y as i8 - to_y as i8).abs();

        shift_x == 2 && shift_y == 1 || shift_x == 1 && shift_y == 2
    }

    // Validity for Bishop piece
    fn validate_bishop(&self, shift: Move) -> bool {
        // Define shifts
        let move_y_axis = shift.destination.y as i8 - shift.origin.y as i8;
        let move_x_axis = shift.destination.x as i8 - shift.origin.x as i8;

        // Compare absolute values for x and y axis shifts
        let is_same_shift: bool = move_x_axis.abs() == move_y_axis.abs();
        let is_moving_x = move_x_axis > 0;
        let is_shifting = (is_moving_x) == (move_y_axis > 0);

        // Manage shift iterators from x and y axis
        let shift_from_x: Range<usize> = shift.origin.x + 1..shift.destination.x;
        let shift_from_y: Range<usize> = shift.origin.y + 1..shift.destination.y;

        // Manage shift iterators to x and y axis
        let shift_to_x: Range<usize> = shift.destination.x + 1..shift.origin.x;
        let shift_to_y: Range<usize> = shift.destination.y + 1..shift.origin.y;

        is_same_shift
            && if is_shifting {
                if is_moving_x {
                    shift_from_x.zip(shift_from_y)
                } else {
                    shift_to_x.zip(shift_to_y)
                }
                .all(|(x, y): (usize, usize)| self.board[y][x] == Cells::Missing)
            } else {
                if is_moving_x {
                    shift_from_x.zip(shift_to_y.rev())
                } else {
                    shift_to_x.zip(shift_from_y.rev())
                }
                .all(|(x, y): (usize, usize)| self.board[y][x] == Cells::Missing)
            }
    }

    // Validity for Pawn piece -> the most fun!
    fn validate_pawn(&self, shift: Move, player: Player, promo: bool) -> bool {
        // Define shifts for origin (from) co-ordinates
        let from_x: usize = shift.origin.x;
        let from_y: usize = shift.origin.y;

        // Define shifts for destination (to) co-ordinates
        let to_x: usize = shift.destination.x;
        let to_y: usize = shift.destination.y;

        // Define matching co-ordinates for black pawns
        let black_x: bool = from_x == to_x && (from_y == to_y + 1 || from_y == 6 && to_y == 4);
        let black_y: bool = (from_y == (to_y + 1)) && (from_x == to_x - 1 || from_x == to_x + 1);

        match player {
            // Is player 1 (white)
            Player::White => {
                // Similar to a C swtich, just pattern matches
                (match self.at(shift.destination) {
                    Cells::Missing => {
                        shift.origin.x == shift.destination.x
                            && (shift.origin.y == shift.destination.y - 1
                                || shift.origin.y == 1 && shift.destination.y == 3)
                    }
                    _ => {
                        shift.origin.y == shift.destination.y - 1
                            && (shift.origin.x == shift.destination.x - 1
                                || shift.origin.x == shift.destination.x + 1)
                    }
                }) && (to_y != 7
                    || promo
                    || self.at(shift.destination) == Cells::Piece(GamePiece::King, Player::Black))
            }
            // Is player 2 (black)
            Player::Black => {
                (match self.at(shift.destination) {
                    Cells::Missing => black_x,
                    _ => black_y,
                }) && (to_y != 0
                    || promo
                    || self.at(shift.destination) == Cells::Piece(GamePiece::King, Player::White))
            }
        }
    }

    // When a pawn reaches the end of the board, they can be promoted to another piece
    fn promotion(&self, shift: Move, player: Player, piece: GamePiece) -> bool {
        // Only allow promotion to a Rook, Knight, Bishop and Queen
        self.at(shift.origin) == Cells::Piece(GamePiece::Pawn, player)
            && match piece {
                // Allow-list
                GamePiece::Rook | GamePiece::Knight | GamePiece::Bishop | GamePiece::Queen => true,
                _ => false,
            }
            // Regular pawn validation
            && self.validate_pawn(shift, player, true)
    }

    // General basic move handling
    fn basic(&self, shift: Move, player: Player) -> bool {
        // Pattern match cell based on co-ordinates
        match self.board[shift.origin.y][shift.origin.x] {
            // Do nothing if cell is empty
            Cells::Missing => false,
            // If the cell has a piece, handle move
            Cells::Piece(piece, cell_player) => {
                cell_player == player
                    // Match destination co-ordinates
                    && match self.at(shift.destination) {
                        Cells::Missing => true,
                        Cells::Piece(_, piece_player) => piece_player != player,
                    }
                    && self.validate_move(piece, shift, cell_player)
            }
        }
    }

    fn validate_king(&self, shift: Move) -> bool {
        // Define shifts for origin (from) co-ordinates
        let from_x: usize = shift.origin.x;
        let from_y: usize = shift.origin.y;

        // Define shifts for destination (to) co-ordinates
        let to_x: usize = shift.destination.x;
        let to_y: usize = shift.destination.y;

        // Simple move validation for nearby cells
        (from_x as i8 - to_x as i8).abs() <= 1 && (from_y as i8 - to_y as i8).abs() <= 1
    }

    // A "check" position in Chess - shows a red square in front-end
    pub fn check(&self, player: Player) -> bool {
        // Define king piece and find previous moves
        let piece = self
            .find_moves(Location {
                // These are base co-ordinates for the king
                x: 4,
                y: match player {
                    Player::White => 0,
                    Player::Black => 7,
                },
            })
            .unwrap();

        // Get current co-ordinates compared to opposite player
        self.get_coordinates(player.other())
            .iter()
            .any(|&location| {
                self.basic(
                    Move {
                        origin: location,
                        destination: piece,
                    },
                    player.other(),
                )
            })
    }

    // A "check mate" position in Chess - ends the game in front-end
    pub fn mate(&self, player: Player) -> bool {
        // Get co-ordinates for player
        self.get_coordinates(player).iter().all(|&location| {
            // Obtain all possible moves and verify whether there is an escape!
            (0usize..8).all(|x: usize| {
                (0usize..8).all(|y: usize| {
                    // If none are validated, it's check mate.
                    !self.validate(
                        Some(MoveType::Normal(Move {
                            origin: location,
                            destination: Location { x, y },
                        })),
                        player,
                    )
                })
            })
        })
    }

    // Return co-ordinates for player
    fn get_coordinates(&self, player: Player) -> Vec<Location> {
        (0usize..8)
            .flat_map(|x: usize| {
                (match player {
                    Player::White => 0usize..2,
                    Player::Black => 6usize..8,
                })
                .map(move |y: usize| Location { x, y })
            })
            // Fetch relevant player moves
            .filter_map(|location| self.find_moves(location))
            // Return elements from iterator as one Vector of co-ordinates
            .collect()
    }

    // Returns available moves for a player's piece
    fn find_moves(&self, co_ordinates: Location) -> Option<Location> {
        self.moves.iter().fold(
            Some(co_ordinates),
            |options: Option<Location>, shift: &Move| match options {
                None => None,
                Some(co_ordinates) => {
                    // Define shifts for origin (from) co-ordinates
                    let from_x: usize = shift.origin.x;
                    let from_y: usize = shift.origin.y;

                    // Define shifts for destination (to) co-ordinates
                    let to_x: usize = shift.destination.x;
                    let to_y: usize = shift.destination.y;

                    // Define matching co-ordinates for iterator
                    let some_from = from_x == co_ordinates.x && from_y == co_ordinates.y;
                    let some_to = to_x == co_ordinates.x && to_y == co_ordinates.y;

                    if some_from {
                        Some(shift.destination)
                    } else if some_to {
                        None
                    } else {
                        Some(co_ordinates)
                    }
                }
            },
        )
    }
}
