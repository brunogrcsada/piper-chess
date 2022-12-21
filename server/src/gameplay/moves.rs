use super::pieces::GamePiece;

#[derive(Copy, Clone)]
pub enum MoveType {
    Normal(Move),
    Promotion(Move, GamePiece),
}

// Convert string input (extremely useful for JSON data streaming) to valid move type
impl MoveType {
    pub fn parse(input: &str) -> Option<MoveType> {
        let mut individual_word = input.split_whitespace();

        match input.split_whitespace().count() {
            // Provides to and from, general move
            2 => match Move::parse(
                individual_word.next().unwrap(),
                individual_word.next().unwrap(),
            ) {
                None => None,
                Some(shift) => Some(MoveType::Normal(shift)),
            },
            // Provides extra arguments for pawn promotion
            4 => {
                // Handle parsing user input for promoting pawns to different pieces
                /* Actual logic for determining which pieces are available is located in board.rs */
                if individual_word.next().unwrap() != "promotion" {
                    None
                } else {
                    // Convert to matching Move and GamePiece objects
                    match (
                        Move::parse(
                            individual_word.next().unwrap(),
                            individual_word.next().unwrap(),
                        ),
                        GamePiece::parse(individual_word.next().unwrap()),
                    ) {
                        (_, None) | (None, _) => None,
                        (Some(mv), Some(piece)) => Some(MoveType::Promotion(mv, piece)),
                    }
                }
            }
            _ => None,
        }
    }
}

// Definition for a 'shift' -> moving a piece from a co-ordinate to another
#[derive(Copy, Clone, PartialEq)]
pub struct Move {
    pub origin: Location,      // previous y, x
    pub destination: Location, // next y, x
}

impl Move {
    // Parser for game piece shift
    fn parse(origin: &str, destination: &str) -> Option<Move> {
        // Pattern match by parsed origin and destination values
        match (Location::parse(origin), Location::parse(destination)) {
            (_, None) | (None, _) => None,
            (Some(origin), Some(destination)) => Some(Move {
                origin,
                destination,
            }),
        }
    }
}

// Base pointer for each cell's location
#[derive(Copy, Clone, PartialEq)]
pub struct Location {
    pub y: usize,
    pub x: usize,
}

// Implement Location struct with a parser
impl Location {
    fn parse(input: &str) -> Option<Location> {
        // There should only be two arguments available
        if input.len() != 2 {
            None
        } else {
            // Extract x and y co-ordinates from input characters
            let (x_value, y_value) = (input.chars().nth(0).unwrap(), input.chars().nth(1).unwrap());

            // Verify if characters match valid tiles
            if x_value < 'a' || x_value > 'h' || y_value < '1' || y_value > '8' {
                None
            } else {
                Some(Location {
                    x: x_value.to_digit(20).unwrap() as usize - 10,
                    y: y_value.to_digit(10).unwrap() as usize - 1,
                })
            }
        }
    }
}
