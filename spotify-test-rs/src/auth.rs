use serde::Deserialize;
use crate::ClientConfig;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
}

pub async fn get_token(cfg:ClientConfig) -> String {
    let client = reqwest::Client::new();
    let auth_res = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(cfg.id, Some(cfg.secret))
        .form(&[ ("grant_type", "client_credentials") ])
        .send()
        .await
        .unwrap()
        .json::<AuthResponse>()
        .await;
    auth_res.unwrap().access_token
}
