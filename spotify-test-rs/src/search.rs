use crate::types;
use std::process;
use reqwest;
use reqwest::StatusCode;

pub async fn search(url: String, token: String) {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("AUTHORIZATION", format!("Bearer {token}"))
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap().text().await.unwrap();

    println!("{:#?}", response);

    // match response.status() {
    //     StatusCode::OK => {
    //         match response.json::<types::APIResponse>().await {
    //             Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
    //             Err(e) => {
    //                 eprintln!("Error parsing json response: {e}");
    //                 process::exit(1);
    //             }
    //         }
    //     },
    //     StatusCode::UNAUTHORIZED => {
    //         eprintln!("Token Error: check token, may need new token.");
    //         process::exit(1);
    //     },
    //     _ => {
    //         panic!("Unexpected response status code");
    //     },
    // }
}
fn print_tracks(tracks: Vec<&types::Track>) {
    for track in tracks {
        println!("🔥 {}", track.name);
        println!("💿 {}", track.album.name);
        println!(
            "🕺 {}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("🌎 {}", track.external_urls.spotify);
        println!("---------")
    }
}
