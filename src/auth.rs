use crate::app::Envs;
use anyhow::Result;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Config, Credentials, OAuth, Token};

pub fn init_spotify() -> AuthCodeSpotify {
    let config = Config {
        token_cached: true,
        ..Default::default()
    };

    let oauth = OAuth {
        scopes: scopes!(
            "user-read-playback-state",
            "user-modify-playback-state",
            "user-read-currently-playing",
            "playlist-read-private",
            "playlist-read-collaborative",
            "playlist-modify-private",
            "playlist-modify-public",
            "user-follow-modify",
            "user-follow-read",
            "user-read-playback-position",
            "user-top-read",
            "user-read-recently-played",
            "user-library-modify",
            "user-library-read",
            "user-read-email",
            "user-read-private"
        ),
        redirect_uri: "http://localhost:8888/callback".to_string(),
        ..Default::default()
    };

    let envs = Envs::new().unwrap();

    let creds = Credentials::new(&envs.client_id, &envs.client_secret);
    AuthCodeSpotify::with_config(creds, oauth, config)
}

pub async fn refresh_auth_code(
    spotify: &AuthCodeSpotify,
    token: Token,
) -> Result<(), &'static str> {
    *spotify.token.lock().await.unwrap() = Some(token.clone());
    if token.is_expired() {
        match spotify.refresh_token().await {
            Ok(_) => Ok(()),
            Err(_) => Err("Couldn't refresh token"),
        }
    } else {
        Ok(())
    }
}
