use spotify_test::read_config;
use spotify_test::auth::get_token;
use spotify_test::search::search;

#[tokio::main]
async fn main() {
    let cfg = read_config();
    let token = get_token(cfg);
    let url = format!(
        "https://api.spotify.com/v1/search?q={query}&type=track,artist",
        query = "Cody Johnson"
    );
    search(url, token.await).await;
}
