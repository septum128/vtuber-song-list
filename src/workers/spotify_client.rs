use reqwest::Client;
use serde::Deserialize;

pub struct SpotifyClient {
    client: Client,
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct SearchResponse {
    tracks: TracksObject,
}

#[derive(Deserialize)]
struct TracksObject {
    items: Vec<TrackItem>,
}

#[derive(Deserialize)]
struct TrackItem {
    artists: Vec<ArtistItem>,
}

#[derive(Deserialize)]
struct ArtistItem {
    name: String,
}

impl SpotifyClient {
    /// Creates a new `SpotifyClient` with the given credentials.
    #[must_use]
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client: Client::new(),
            client_id,
            client_secret,
        }
    }

    async fn get_token(&self) -> Result<String, reqwest::Error> {
        let resp: TokenResponse = self
            .client
            .post("https://accounts.spotify.com/api/token")
            .basic_auth(&self.client_id, Some(&self.client_secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp.access_token)
    }

    /// Searches for an artist name by song title using two query patterns.
    /// Returns the first found artist name, or `None` if not found.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network or JSON parse failure.
    pub async fn search_artist(&self, title: &str) -> Result<Option<String>, reqwest::Error> {
        let token = self.get_token().await?;

        // Try two patterns: title only, then title + "track"
        let queries = [title.to_string(), format!("{title} track")];

        for query in &queries {
            let encoded = urlencoding_encode(query);
            let url = format!("https://api.spotify.com/v1/search?q={encoded}&type=track&limit=1");
            let resp: SearchResponse = self
                .client
                .get(&url)
                .bearer_auth(&token)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            if let Some(track) = resp.tracks.items.into_iter().next() {
                if let Some(artist) = track.artists.into_iter().next() {
                    return Ok(Some(artist.name));
                }
            }
        }

        Ok(None)
    }
}

fn urlencoding_encode(s: &str) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push('%');
                let _ = write!(encoded, "{byte:02X}");
            }
        }
    }
    encoded
}
