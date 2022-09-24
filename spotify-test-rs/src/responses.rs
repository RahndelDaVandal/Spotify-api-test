use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIResponse {
    pub tracks: Items<Track>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items<T> {
    pub items: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<Box<LinkedFrom>>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: Option<i32>,
    pub preview_url: Option<String>,
    pub track_number: i32,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Album {
    pub album_type: String,
    pub total_tracks: i32,
    pub available_markets: Vec<String>,
    pub external_urls: ExternalUrls,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    pub release_date: String,
    pub release_date_precision: String,
    pub restrictions: Option<Restrictions>,
    pub uri: String,
    pub album_group: Option<String>,
    pub artists: Vec<Artist>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Image {
    pub url: String,
    pub height: i32,
    pub width:i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub external_urls: ExternalUrls,
    pub followers: Option<Followers>,
    pub genres: Option<Vec<String>>,
    pub href: String,
    pub id: String,
    pub images: Option<Vec<Image>>,
    pub name: String,
    pub popularity: Option<i32>,
    pub uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalIds{
    isrc: Option<String>,
    ean: Option<String>,
    upc: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExternalUrls {
    pub spotify: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkedFrom {
    pub album: Album,
    pub artists: Vec<Artist>,
    pub available_markets: Vec<String>,
    pub disc_number: i32,
    pub duration_ms: i32,
    pub explicit: bool,
    pub external_ids: ExternalIds,
    pub external_urls: ExternalUrls,
    pub id: String,
    pub is_playable: Option<bool>,
    pub linked_from: Option<Box<LinkedFrom>>,
    pub restrictions: Option<Restrictions>,
    pub name: String,
    pub popularity: Option<i32>,
    pub preview_url: String,
    pub track_number: i32,
    pub uri: String,
    pub is_local: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Restrictions {
    reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Followers {
    href: Option<String>,
    total: Option<i32>,
}
