#![allow(dead_code)]

use spotify_oauth::{SpotifyAuth, SpotifyCallback, SpotifyScope};
use std::{error::Error, io::stdin, str::FromStr};

mod spotify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    println!("Attempting to open Spotify Authorization URL...");

    let auth = SpotifyAuth::new(
        "CLIENT_ID".into(),
        "CLIENT_SECRET".into(),
        "code".into(),
        "http://spotify-authorization.000webhostapp.com/spotify.php".into(),
        vec![SpotifyScope::Streaming],
        false,
    );
    let auth_url = auth.authorize_url();

    if auth_url.is_err() {
        panic!("Error/None: {:?}", auth_url.err());
    }

    open::that(auth_url.unwrap())?;

    println!("Authorization URL opened, waiting for callback... (paste the URL here)");

    let mut buffer = String::new();

    stdin().read_line(&mut buffer)?;

    let spotify_token = SpotifyCallback::from_str(buffer.trim())?
        .convert_into_token(auth.client_id, auth.client_secret, auth.redirect_uri)
        .await?;

    println!("Spotify authorization completed {:?}", spotify_token);

    // Do something with the token

    Ok(())
}
