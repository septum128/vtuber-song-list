use reqwest::Client;
use serde::Serialize;

pub struct SlackClient {
    client: Client,
    webhook_url: String,
}

#[derive(Serialize)]
struct SlackMessage<'a> {
    text: &'a str,
}

impl SlackClient {
    /// Creates a new `SlackClient` with the given webhook URL.
    #[must_use]
    pub fn new(webhook_url: String) -> Self {
        Self {
            client: Client::new(),
            webhook_url,
        }
    }

    /// Sends a message to the configured Slack webhook.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network failure.
    pub async fn notify(&self, message: &str) -> Result<(), reqwest::Error> {
        self.client
            .post(&self.webhook_url)
            .json(&SlackMessage { text: message })
            .send()
            .await?;
        Ok(())
    }
}
