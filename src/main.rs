use rspotify::{clients::OAuthClient, AuthCodeSpotify, Token};
use rustify::auth::*;
use std::{error::Error, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let spotify = init_spotify();
    let token = Token::from_cache(&spotify.config.cache_path);

    match token {
        Ok(_) => {
            // Check if token is expired and refresh it if needed
            let _ = refresh_auth_code(&spotify).await;
        }
        Err(_) => {
            let _ = get_token_auto(&spotify).await;
        }
    }

    let token = spotify.read_token_cache(false).await.unwrap();
    *spotify.token.lock().await.unwrap() = token;

    println!("{:?}", spotify.pause_playback(None).await);

    Ok(())
}

async fn get_token_auto(spotify: &AuthCodeSpotify) -> Result<(), &'static str> {
    // Manual auth for now, in the future auto auth should be implemented
    let url = spotify.get_authorize_url(true).unwrap();
    println!("Open the following link in the browser and paste in the response URL:");
    println!("{}", url);

    let mut input = "".to_string();

    let _ = std::io::stdout().flush();

    if let Ok(_) = std::io::stdin().read_line(&mut input) {
        let code = spotify.parse_response_code(&input).unwrap();
        match spotify.request_token(&code).await {
            Ok(_) => return Ok(()),
            Err(_) => return Err("Error requesting token"),
        }
    } else {
        return Err("Couldn't parse URL");
    }
}
