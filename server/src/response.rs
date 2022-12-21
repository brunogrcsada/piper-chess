use std::collections::HashMap;

use crate::gameplay::players::Player;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

// Type definition for the JSON object that is returned to the client
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct FinalResponse {
    pub player: String,                 // Player colour
    pub world: HashMap<String, String>, // World shape
    pub valid: bool,                    // Is the move made valid
    pub check: bool,                    // Is the player in a check position
    pub mate: bool,                     // Is the player in a check-mate position
}

// Server error message (a json object with an error key)
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct StatusBuilder {
    pub error: Error,
}

// Full definition for sending the error to the client
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Error {
    pub code: String,
    pub message: String,
}

// Send data back to the client
pub async fn send(
    color: &Player,
    world: HashMap<String, String>,
    valid: bool,
    check: bool,
    mate: bool,
) -> HttpResponse {
    HttpResponse::Ok().json(FinalResponse {
        player: color.display().to_string(),
        world,
        valid,
        check,
        mate,
    })
}
