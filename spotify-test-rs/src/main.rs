#![allow(unused_imports)]
use spotify_test::read_config; use spotify_test::auth::get_token;
use spotify_test::search::search;
use spotify_test::search::debug_search;

#[tokio::main]
async fn main() {
    env_logger::init();
    let cfg = read_config();
    let token = get_token(cfg);
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = "Cody Johnson"
    );
    // debug_search(url, token.await).await;
    search(url, token.await).await;
}
