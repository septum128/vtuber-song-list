use loco_rs::prelude::*;

use super::cloudflare_client::CloudflareEmailClient;

pub struct CloudflareMailerWorker {
    pub ctx: AppContext,
}

#[async_trait]
impl BackgroundWorker<mailer::Email> for CloudflareMailerWorker {
    fn build(ctx: &AppContext) -> Self {
        Self { ctx: ctx.clone() }
    }

    async fn perform(&self, email: mailer::Email) -> Result<()> {
        let account_id =
            std::env::var("CLOUDFLARE_ACCOUNT_ID").map_err(|e| Error::Any(Box::new(e)))?;
        let api_token =
            std::env::var("CLOUDFLARE_API_TOKEN").map_err(|e| Error::Any(Box::new(e)))?;

        let client = CloudflareEmailClient::new(account_id, api_token);
        client
            .send(&email)
            .await
            .map_err(|e| Error::Any(Box::new(e)))?;

        Ok(())
    }
}
