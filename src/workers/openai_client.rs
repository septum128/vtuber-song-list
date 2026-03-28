use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

pub struct SetlistEntry {
    pub time: String,
    pub title: String,
    pub author: String,
}

#[derive(Serialize)]
struct ChatRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
}

#[derive(Serialize)]
struct ChatMessage<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Deserialize)]
struct ChatResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct RawSetlistEntry {
    time: Option<String>,
    title: Option<String>,
    author: Option<String>,
}

impl OpenAIClient {
    /// Creates a new `OpenAIClient` with the given API key.
    #[must_use]
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    /// Extracts a setlist from a comment using `OpenAI` `gpt-4o-mini`.
    /// Returns a list of `SetlistEntry` with time normalized to `HH:MM:SS`.
    ///
    /// # Errors
    /// Returns a `String` error message on network, API, or JSON parse failure.
    pub async fn extract_setlist(&self, comment: &str) -> Result<Vec<SetlistEntry>, String> {
        let system_prompt = "コメント本文からタイムスタンプ付き楽曲リストを抽出して\
            [{\"time\":\"HH:MM:SS\",\"title\":\"曲名\",\"author\":\"アーティスト名\"}] \
            形式のJSONのみを返す。timeは必ずHH:MM:SS形式に正規化すること（MM:SS → 00:MM:SS）。\
            authorが不明な場合は空文字列を使用すること。";

        let request = ChatRequest {
            model: "gpt-4o-mini",
            messages: vec![
                ChatMessage {
                    role: "system",
                    content: system_prompt,
                },
                ChatMessage {
                    role: "user",
                    content: comment,
                },
            ],
        };

        let resp = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let chat_resp: ChatResponse = resp.json().await.map_err(|e| e.to_string())?;

        let content = chat_resp
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| "No choices in OpenAI response".to_string())?;

        // Strip markdown code fences if present
        let json_str = content
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim();

        let raw: Vec<RawSetlistEntry> =
            serde_json::from_str(json_str).map_err(|e| e.to_string())?;

        let entries = raw
            .into_iter()
            .map(|r| SetlistEntry {
                time: normalize_time(r.time.unwrap_or_default().trim()),
                title: r.title.unwrap_or_default(),
                author: r.author.unwrap_or_default(),
            })
            .collect();

        Ok(entries)
    }
}

/// Normalizes a time string to `HH:MM:SS`.
/// `MM:SS` becomes `00:MM:SS`, `HH:MM:SS` is left unchanged.
fn normalize_time(time: &str) -> String {
    let parts: Vec<&str> = time.split(':').collect();
    match parts.len() {
        2 => format!("00:{}:{}", parts[0], parts[1]),
        _ => time.to_string(),
    }
}
