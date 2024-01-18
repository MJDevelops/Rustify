use rspotify::{ClientCredsSpotify, Credentials};
use std::env;

pub async fn get_client_credentials() -> Result<ClientCredsSpotify, &'static str> {
    let client_id: String = if let Ok(val) = env::var("CLIENT_ID") {
        val
    } else {
        return Err("Couldn't get client id");
    };

    let client_secret: String = if let Ok(val) = env::var("CLIENT_SECRET") {
        val
    } else {
        return Err("Couldn't get client secret");
    };

    let client_creds = Credentials::new(&client_id, &client_secret);
    let client_creds_spotify = ClientCredsSpotify::new(client_creds);

    match client_creds_spotify.request_token().await {
        Ok(_) => (),
        Err(_) => return Err("Couldn't request token"),
    };
    Ok(client_creds_spotify)
}
