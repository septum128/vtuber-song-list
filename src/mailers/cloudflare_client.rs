use loco_rs::mailer;
use reqwest::Client;
use serde::Serialize;

const SEND_ENDPOINT: &str = "https://api.cloudflare.com/client/v4/accounts";

pub struct CloudflareEmailClient {
    client: Client,
    account_id: String,
    api_token: String,
}

#[derive(Serialize)]
struct Address {
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize)]
struct SendRequest {
    to: String,
    from: Address,
    subject: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bcc: Option<String>,
    #[serde(rename = "reply_to", skip_serializing_if = "Option::is_none")]
    reply_to: Option<String>,
}

/// Parses a loco-style mailbox string (`"Name <addr@example.com>"` or a bare
/// address) into the `{address, name}` shape Cloudflare's REST API expects.
fn parse_mailbox(raw: &str) -> Address {
    if let (Some(lt), Some(gt)) = (raw.find('<'), raw.find('>')) {
        if lt < gt {
            let name = raw[..lt].trim().trim_matches('"').to_string();
            let address = raw[lt + 1..gt].trim().to_string();
            return Address {
                address,
                name: if name.is_empty() { None } else { Some(name) },
            };
        }
    }

    Address {
        address: raw.trim().to_string(),
        name: None,
    }
}

impl CloudflareEmailClient {
    #[must_use]
    pub fn new(account_id: String, api_token: String) -> Self {
        Self {
            client: Client::new(),
            account_id,
            api_token,
        }
    }

    /// Sends `email` via the Cloudflare Email Sending REST API.
    ///
    /// # Errors
    /// Returns `reqwest::Error` on network failure or a non-2xx response.
    pub async fn send(&self, email: &mailer::Email) -> Result<(), reqwest::Error> {
        let from = parse_mailbox(email.from.as_deref().unwrap_or(mailer::DEFAULT_FROM_SENDER));

        let request = SendRequest {
            to: email.to.clone(),
            from,
            subject: email.subject.clone(),
            html: (!email.html.is_empty()).then(|| email.html.clone()),
            text: (!email.text.is_empty()).then(|| email.text.clone()),
            cc: email.cc.clone(),
            bcc: email.bcc.clone(),
            reply_to: email.reply_to.clone(),
        };

        self.client
            .post(format!(
                "{SEND_ENDPOINT}/{}/email/sending/send",
                self.account_id
            ))
            .bearer_auth(&self.api_token)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::parse_mailbox;

    #[test]
    fn parses_name_and_address() {
        let addr = parse_mailbox("System <system@example.com>");
        assert_eq!(addr.address, "system@example.com");
        assert_eq!(addr.name.as_deref(), Some("System"));
    }

    #[test]
    fn parses_bare_address() {
        let addr = parse_mailbox("system@example.com");
        assert_eq!(addr.address, "system@example.com");
        assert_eq!(addr.name, None);
    }
}
