use std::env;
use rspotify::{Credentials, ClientCredsSpotify};

fn get_client_credentials() -> Result<ClientCredsSpotify, &'static str> {
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

    Ok(client_creds_spotify) 
}

