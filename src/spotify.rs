use serde_json::Value;

#[derive(serde::Deserialize, Debug)]
pub struct SpotifyPlaylist {
    collaborative: bool,
    description: String,
    external_urls: Value,
    followers: Value,
    href: String,
    id: String,
    images: Vec<Value>,
    name: String,
    owner: Value,
    public: bool,
    snapshot_id: String,
    tracks: SpotifyTracks,
    r#type: String,
    uri: String,
}

#[derive(serde::Deserialize, Debug)]
struct SpotifyTracks {
    href: String,
    limit: i32,
    next: String,
    offset: i32,
    previous: String,
    total: i32,
    items: Vec<SpotifySong>,
}

#[derive(serde::Deserialize, Debug)]
struct SpotifySong {
    added_at: String,
    added_by: Value,
    is_local: bool,
    primary_color: Value,
    track: TrackDetails,
    video_thumbnail: Value,
}

#[derive(serde::Deserialize, Debug)]
struct TrackDetails {
    album: Value,
    artists: Vec<Value>,
    available_markets: Vec<String>,
    disc_number: i32,
    duration_ms: i32,
    explicit: bool,
    external_ids: Value,
    external_urls: Value,
    href: String,
    id: String,
    is_local: bool,
    is_playable: bool,
    linked_from: Value,
    restrictions: Value,
    name: String,
    popularity: i32,
    preview_url: String,
    r#type: String,
    uri: String,
}
