mod gameplay;
mod response;

use std::collections::HashMap;

use gameplay::board::ChessBoard;
use gameplay::moves::MoveType;
use gameplay::players::Player;

#[cfg(test)]
mod tests;

use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer};

static mut GAME_BOARD: ChessBoard = ChessBoard::new();
static mut CURRENT_PLAYER: Player = Player::White;

// {from} and {to} are part of the path, and state piece movements
#[get("/move/{from}/{to}")]
async fn index(moves: web::Path<(String, String)>) -> HttpResponse {
    println!("{} {}", moves.0, moves.1);

    // General game
    let data: HashMap<String, String>;
    let player: Player;

    // Valid move tracker
    let mut valid: bool = true;

    // Check + mate tracker
    let mut check: bool = false;
    let mut mate: bool = false;

    // Not currently using any database to store game state, so using global state
    unsafe {
        let options = MoveType::parse(&[moves.0.to_string(), moves.1.to_string()].join(" "));

        // Validate move passed in request
        if !GAME_BOARD.validate(options, CURRENT_PLAYER) {
            // The move is not valid
            valid = false;
        } else {
            // If the move is valid, apply it
            GAME_BOARD = GAME_BOARD.apply_move(options.unwrap());

            //End of turn logic
            CURRENT_PLAYER = CURRENT_PLAYER.other();
        }

        player = CURRENT_PLAYER;

        if GAME_BOARD.check(CURRENT_PLAYER) {
            check = true;
            if GAME_BOARD.mate(CURRENT_PLAYER) {
                mate = true;
            }
        }

        data = GAME_BOARD.output();
    }

    // Send back response to client
    response::send(&player, data, valid, check, mate).await
}

#[get("/state")]
async fn state() -> HttpResponse {
    // General game
    let data: HashMap<String, String>;
    let color: Player;

    // Valid move tracker
    let valid: bool = true;

    // Check + mate tracker
    let mut check: bool = false;
    let mut mate: bool = false;

    // Not currently using any database to store game state, so using global state
    unsafe {
        data = GAME_BOARD.output();

        if GAME_BOARD.check(CURRENT_PLAYER) {
            check = true;
            if GAME_BOARD.mate(CURRENT_PLAYER) {
                mate = true;
            }
        }

        color = CURRENT_PLAYER;
    }

    // Send back response to client
    response::send(&color, data, valid, check, mate).await
}

// Start web server with CORS configuration, on HOST 0.0.0.0 and PORT 2020
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let url = "0.0.0.0:2020";
    println!("Piper chess started on {}", url);

    HttpServer::new(move || {
        let cors = Cors::default()
            // Specify which addresses the server can be accessed from
            .allowed_origin("http://localhost:3001")
            .allowed_origin("http://localhost:2020")
            .allowed_origin("http://localhost:4221")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT]);
        App::new().wrap(cors).service(index).service(state)
    })
    .bind(url)?
    .run()
    .await
}
