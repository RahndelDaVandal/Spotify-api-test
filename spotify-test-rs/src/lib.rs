use dotenv::dotenv;
use std::{env,process};

pub mod responses;
pub mod search;
pub mod auth;

pub fn read_config() -> ClientConfig {
    dotenv().ok();
    ClientConfig {
        id: match env::var("SPOTIFY_CLIENT_ID") {
            Ok(i) => {i},
            Err(e) => {
                // TODO: Expand errror
                eprintln!("Error: {e}");
                process::exit(1);
            },
        },
        secret: match env::var("SPOTIFY_CLIENT_SECRET") {
            Ok(s) => {s},
            Err(e) => {
                // TODO: Expand errror
                eprintln!("Error: {e}");
                process::exit(1);
            },
        },
    }
    // match env::var("SPOTIFY_TOKEN") {
    //     Ok(token) => {
    //         token
    //     },
    //     Err(_) => {
    //         eprintln!("ERROR: 'SPOTIFY_TOKEN' not found in enviroment variables");
    //         process::exit(1);
    //     }
    // }
}

pub struct ClientConfig {
    pub id: String,
    pub secret: String,
}
