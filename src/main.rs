use rspotify::{clients::OAuthClient, AuthCodeSpotify};
use rustify::{app, auth::*};
use std::{error::Error, io::Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let spotify = init_spotify();
    let _ = get_token_auto(&spotify).await;

    println!("{:?}", *spotify.token.lock().await.unwrap());

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
        let code = spotify.parse_response_code(&url).unwrap();
        match spotify.request_token(&code).await {
            Ok(_) => return Ok(()),
            Err(e) => return Err("Error requesting token"),
        }
    } else {
        return Err("Couldn't parse URL");
    }
}
