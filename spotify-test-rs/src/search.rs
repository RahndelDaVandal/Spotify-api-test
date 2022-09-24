use crate::responses;
use std::process;
use reqwest;
use reqwest::StatusCode;

pub async fn debug_search(url: String, token: String) {
    log::debug!("search::search({}, token)", url);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("AUTHORIZATION", format!("Bearer {token}"))
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await;
    println!("\n<======= RESPONSE =======>\n{}", response.unwrap());
}

pub async fn search(url: String, token: String) {
    log::debug!("search::search({}, token)", url);
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("AUTHORIZATION", format!("Bearer {token}"))
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .unwrap();

    // println!("{:#?}", response);
    log::debug!("\n<======= RESPONSE =======>\n{:#?}", response.status());

    match response.status() {
        StatusCode::OK => {
            match response.json::<responses::APIResponse>().await {
                // Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
                Ok(parsed) => {
                    for t in parsed.tracks.items {
                        println!("🎵 {}\n💽 {}\n", t.name, t.album.name);
                    }
                },
                Err(e) => {
                    eprintln!("Error parsing json response: {e}");
                    process::exit(1);
                }
            }
        },
        StatusCode::UNAUTHORIZED => {
            eprintln!("Token Error: check token, may need new token.");
            process::exit(1);
        },
        _ => {
            panic!("Unexpected response status code");
        },
    }
}

// #[allow(dead_code)]
// fn print_tracks(tracks: Vec<&responses::Track>) {
//     for track in tracks {
//         println!("🔥 {}", track.name);
//         println!("💿 {}", track.album.name);
//         println!(
//             "🕺 {}",
//             track
//                 .album
//                 .artists
//                 .iter()
//                 .map(|artist| artist.name.to_string())
//                 .collect::<String>()
//         );
//         println!("🌎 {}", track.external_urls.spotify);
//         println!("---------")
//     }
// }
