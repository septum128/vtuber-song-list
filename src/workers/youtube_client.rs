use regex::Regex;
use reqwest::Client;
use serde::Deserialize;

pub struct YouTubeClient {
    client: Client,
    api_key: String,
}

pub struct RssEntry {
    pub video_id: String,
    pub title: String,
    pub published: String,
}

pub struct VideoInfo {
    pub video_id: String,
    pub title: String,
    pub published_at: String,
    pub response_json: serde_json::Value,
}

pub struct CommentInfo {
    pub comment_id: String,
    pub author: String,
    pub content: String,
    pub response_json: serde_json::Value,
}

#[derive(Deserialize)]
struct ChannelListResponse {
    items: Vec<ChannelItem>,
}

#[derive(Deserialize)]
struct ChannelItem {
    snippet: ChannelSnippet,
}

#[derive(Deserialize)]
struct ChannelSnippet {
    thumbnails: Thumbnails,
}

#[derive(Deserialize)]
struct Thumbnails {
    default: ThumbnailEntry,
}

#[derive(Deserialize)]
struct ThumbnailEntry {
    url: String,
}

#[derive(Deserialize)]
struct VideoListResponse {
    items: Vec<VideoItem>,
}

#[derive(Deserialize)]
struct VideoItem {
    id: String,
    snippet: VideoSnippet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct VideoSnippet {
    title: String,
    published_at: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentThreadListResponse {
    next_page_token: Option<String>,
    items: Vec<CommentThreadItem>,
}

#[derive(Deserialize)]
struct CommentThreadItem {
    snippet: CommentThreadSnippet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentThreadSnippet {
    top_level_comment: TopLevelComment,
}

#[derive(Deserialize)]
struct TopLevelComment {
    id: String,
    snippet: CommentSnippet,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentSnippet {
    author_display_name: String,
    text_display: String,
}

impl YouTubeClient {
    /// Creates a new `YouTubeClient` with the given API key.
    #[must_use]
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Fetches the default thumbnail URL for a `YouTube` channel.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network or JSON parse failure.
    pub async fn fetch_channel_icon(
        &self,
        channel_id: &str,
    ) -> Result<Option<String>, reqwest::Error> {
        let url = format!(
            "https://www.googleapis.com/youtube/v3/channels\
             ?part=snippet&id={channel_id}&key={}",
            self.api_key
        );
        let resp: ChannelListResponse = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(resp
            .items
            .into_iter()
            .next()
            .map(|item| item.snippet.thumbnails.default.url))
    }

    /// Fetches the latest video entries from a channel's `YouTube` RSS feed.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network or parse failure.
    pub async fn fetch_rss_entries(
        &self,
        channel_id: &str,
    ) -> Result<Vec<RssEntry>, reqwest::Error> {
        let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={channel_id}");
        let body = self.client.get(&url).send().await?.text().await?;
        Ok(parse_rss_entries(&body))
    }

    /// Fetches video info for the given video IDs from the `YouTube` Data API v3.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network or JSON parse failure.
    pub async fn fetch_video_info(
        &self,
        video_ids: &[&str],
    ) -> Result<Vec<VideoInfo>, reqwest::Error> {
        let ids = video_ids.join(",");
        let url = format!(
            "https://www.googleapis.com/youtube/v3/videos?part=snippet&id={ids}&key={}",
            self.api_key
        );
        let resp: VideoListResponse = self
            .client
            .get(&url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let items = resp
            .items
            .into_iter()
            .map(|item| {
                let snippet_json =
                    serde_json::to_value(&item.snippet).unwrap_or(serde_json::Value::Null);
                VideoInfo {
                    video_id: item.id,
                    title: item.snippet.title,
                    published_at: item.snippet.published_at,
                    response_json: snippet_json,
                }
            })
            .collect();
        Ok(items)
    }

    /// Fetches all comments for the given video from the `YouTube` Data API v3.
    /// Handles pagination automatically.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network or JSON parse failure.
    pub async fn fetch_comments(&self, video_id: &str) -> Result<Vec<CommentInfo>, reqwest::Error> {
        let mut results = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let mut url = format!(
                "https://www.googleapis.com/youtube/v3/commentThreads\
                 ?part=snippet&videoId={video_id}&maxResults=100&key={}",
                self.api_key
            );
            if let Some(ref token) = page_token {
                url.push_str("&pageToken=");
                url.push_str(token);
            }

            let resp: CommentThreadListResponse = self
                .client
                .get(&url)
                .send()
                .await?
                .error_for_status()?
                .json()
                .await?;

            for item in resp.items {
                let top = item.snippet.top_level_comment;
                let snippet_json =
                    serde_json::to_value(&top.snippet).unwrap_or(serde_json::Value::Null);
                results.push(CommentInfo {
                    comment_id: top.id,
                    author: top.snippet.author_display_name,
                    content: top.snippet.text_display,
                    response_json: snippet_json,
                });
            }

            match resp.next_page_token {
                Some(token) => page_token = Some(token),
                None => break,
            }
        }

        Ok(results)
    }
}

fn parse_rss_entries(xml: &str) -> Vec<RssEntry> {
    let video_id_re =
        Regex::new(r"<yt:videoId>([^<]+)</yt:videoId>").expect("static regex is valid");
    let title_re = Regex::new(r"<title>([^<]+)</title>").expect("static regex is valid");
    let published_re =
        Regex::new(r"<published>([^<]+)</published>").expect("static regex is valid");

    let video_ids: Vec<&str> = video_id_re
        .captures_iter(xml)
        .filter_map(|c| c.get(1).map(|m| m.as_str()))
        .collect();

    // Titles: first match is channel title, subsequent are video titles
    let titles: Vec<&str> = title_re
        .captures_iter(xml)
        .filter_map(|c| c.get(1).map(|m| m.as_str()))
        .skip(1) // skip channel title
        .collect();

    let published_dates: Vec<&str> = published_re
        .captures_iter(xml)
        .filter_map(|c| c.get(1).map(|m| m.as_str()))
        .collect();

    video_ids
        .into_iter()
        .enumerate()
        .map(|(i, vid)| RssEntry {
            video_id: vid.to_string(),
            title: titles.get(i).map_or("", |s| *s).to_string(),
            published: published_dates.get(i).map_or("", |s| *s).to_string(),
        })
        .collect()
}

// Make VideoSnippet serializable for response_json
impl serde::Serialize for VideoSnippet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("title", &self.title)?;
        map.serialize_entry("publishedAt", &self.published_at)?;
        map.end()
    }
}

// Make CommentSnippet serializable for response_json
impl serde::Serialize for CommentSnippet {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap as _;
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("authorDisplayName", &self.author_display_name)?;
        map.serialize_entry("textDisplay", &self.text_display)?;
        map.end()
    }
}
