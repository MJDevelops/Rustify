use rustify::auth::get_client_credentials;
use std::{
    error::Error,
    process::exit,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let creds = get_client_credentials().await.unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1);
    });

    let token = creds.token.lock().await.unwrap();
    let access_token: Option<&str> = match token.as_ref() {
        Some(val) => Some(&val.access_token),
        None => None,
    };

    if let Some(val) = access_token {
        println!("{}", val);
    }

    Ok(())
}
