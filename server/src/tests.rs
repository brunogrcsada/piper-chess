use std::path::{Path, PathBuf};

pub fn mock() -> PathBuf {
    std::env::current_dir().unwrap().join(Path::new("mocks"))
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct Response {
    player: String,
    valid: bool,
    check: bool,
    mate: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use serde_json::Value;
    use std::fs;

    // --------------------Mocking:-----------------------

    /* A quick example on how we can mock a request to the chess API.
    In the example Mock, we have the global state of the game, including
    whether the current move is valid, whether the player is checked,
    or whether a check mate has occurred. The following test compares
    the mock to the server body returned at the start of a match */

    #[tokio::test]
    async fn mock_should_return_game() {
        // Sample Data:
        let player: &str = "white";
        let valid: bool = true;
        let check: bool = false;
        let mate: bool = false;

        // Start Mock server and retrieve mock file with sample chess board
        let server: MockServer = MockServer::start().await;

        let mock_data: String = fs::read_to_string(mock().join("mock.json")).unwrap();
        let body_val: Value = serde_json::from_str(&mock_data).unwrap();
        let response: ResponseTemplate = ResponseTemplate::new(200).set_body_json(body_val);

        // Create local URL for request
        let request_url = format!("{}{}", &server.uri(), "/state");

        // Run mock
        Mock::given(method("GET"))
            .and(path("state"))
            .respond_with(response)
            .mount(&server)
            .await;

        let resp = reqwest::get(request_url).await.unwrap();

        match resp.status() {
            reqwest::StatusCode::OK => {
                // Parse request through Response struct
                match resp.json::<Response>().await {
                    Ok(parsed) => {
                        assert_eq!(player.to_string(), parsed.player);
                        assert_eq!(valid, parsed.valid);
                        assert_eq!(check, parsed.check);
                        assert_eq!(mate, parsed.mate);
                    }
                    Err(_) => println!("Incorrect response body, unable to pass."),
                };
            }
            other => {
                panic!("Unknown error: {:?}", other);
            }
        };
    }
}
