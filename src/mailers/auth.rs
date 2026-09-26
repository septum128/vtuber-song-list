// auth mailer
use loco_rs::{environment::Environment, prelude::*};
use serde_json::json;

use super::cloudflare_worker::CloudflareMailerWorker;
use crate::models::users;

const WELCOME_SUBJECT: &str = include_str!("auth/welcome/subject.t");
const WELCOME_HTML: &str = include_str!("auth/welcome/html.t");
const WELCOME_TEXT: &str = include_str!("auth/welcome/text.t");

const FORGOT_SUBJECT: &str = include_str!("auth/forgot/subject.t");
const FORGOT_HTML: &str = include_str!("auth/forgot/html.t");
const FORGOT_TEXT: &str = include_str!("auth/forgot/text.t");

const MAGIC_LINK_SUBJECT: &str = include_str!("auth/magic_link/subject.t");
const MAGIC_LINK_HTML: &str = include_str!("auth/magic_link/html.t");
const MAGIC_LINK_TEXT: &str = include_str!("auth/magic_link/text.t");

const ADMIN_NOTIFICATION_SUBJECT: &str = include_str!("auth/admin_notification/subject.t");
const ADMIN_NOTIFICATION_HTML: &str = include_str!("auth/admin_notification/html.t");
const ADMIN_NOTIFICATION_TEXT: &str = include_str!("auth/admin_notification/text.t");

/// The public URL users' browsers should hit for links embedded in emails
/// (email verification, password reset, magic link).
///
/// `ctx.config.server.full_url()` (`host:port`) doesn't work behind Railway's
/// edge, where the public domain terminates TLS on 443 and routes internally
/// to a different container port — appending `:{port}` there produces a
/// broken URL. `APP_HOST` is read directly here instead and should be set to
/// the full public origin with no port (e.g.
/// `https://vtuber-song-list-staging.up.railway.app`).
fn public_app_url() -> String {
    std::env::var("APP_HOST").unwrap_or_else(|_| "http://localhost:5150".to_string())
}

/// Renders a single Tera template string against `locals`.
fn render(template: &str, locals: &serde_json::Value, autoescape: bool) -> Result<String> {
    let context = tera::Context::from_serialize(locals).map_err(|e| Error::Any(Box::new(e)))?;
    tera::Tera::one_off(template, &context, autoescape).map_err(|e| Error::Any(Box::new(e)))
}

fn render_email(
    to: String,
    subject_t: &str,
    html_t: &str,
    text_t: &str,
    locals: &serde_json::Value,
) -> Result<mailer::Email> {
    Ok(mailer::Email {
        to,
        subject: render(subject_t, locals, false)?,
        html: render(html_t, locals, true)?,
        text: render(text_t, locals, false)?,
        ..Default::default()
    })
}

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {
    fn opts() -> mailer::MailerOpts {
        mailer::MailerOpts {
            from: std::env::var("CLOUDFLARE_EMAIL_FROM")
                .unwrap_or_else(|_| mailer::DEFAULT_FROM_SENDER.to_string()),
            ..Default::default()
        }
    }
}
impl AuthMailer {
    /// Sends an email through Cloudflare Email Sending when configured
    /// (`CLOUDFLARE_API_TOKEN` set — the case for staging/production), otherwise
    /// falls back to loco's built-in SMTP mailer (used in development/test).
    ///
    /// # Errors
    ///
    /// When email sending is failed
    async fn dispatch(ctx: &AppContext, mut email: mailer::Email) -> Result<()> {
        let opts = Self::opts();
        email.from = Some(email.from.unwrap_or(opts.from));
        email.reply_to = email.reply_to.or(opts.reply_to);

        if std::env::var("CLOUDFLARE_API_TOKEN").is_ok() {
            CloudflareMailerWorker::perform_later(ctx, email).await?;
        } else {
            if !matches!(
                ctx.environment,
                Environment::Development | Environment::Test
            ) {
                tracing::error!(
                    "CLOUDFLARE_API_TOKEN not set outside dev/test — falling back to the SMTP \
                     mailer, which is also unconfigured; email will NOT be sent"
                );
            }
            Self::mail(ctx, &email).await?;
        }

        Ok(())
    }

    /// Sending welcome email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let email = render_email(
            user.email.clone(),
            WELCOME_SUBJECT,
            WELCOME_HTML,
            WELCOME_TEXT,
            &json!({
              "name": user.name,
              "verifyToken": user.email_verification_token,
              "domain": public_app_url()
            }),
        )?;

        Self::dispatch(ctx, email).await
    }

    /// Sending forgot password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let email = render_email(
            user.email.clone(),
            FORGOT_SUBJECT,
            FORGOT_HTML,
            FORGOT_TEXT,
            &json!({
              "name": user.name,
              "resetToken": user.reset_token,
              "domain": public_app_url()
            }),
        )?;

        Self::dispatch(ctx, email).await
    }

    /// Sends a magic link authentication email to the user.
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_magic_link(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let token = user
            .magic_link_token
            .clone()
            .ok_or_else(|| Error::string("the user model not contains magic link token"))?;

        let email = render_email(
            user.email.clone(),
            MAGIC_LINK_SUBJECT,
            MAGIC_LINK_HTML,
            MAGIC_LINK_TEXT,
            &json!({
              "name": user.name,
              "token": token,
              "host": public_app_url()
            }),
        )?;

        Self::dispatch(ctx, email).await
    }

    /// Notifies the admin (via `ADMIN_NOTIFICATION_EMAIL`) that a new user registered.
    /// Does nothing if the env var is not set, since this is an optional feature.
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn notify_admin_of_registration(ctx: &AppContext, user: &users::Model) -> Result<()> {
        let Some(admin_email) = std::env::var("ADMIN_NOTIFICATION_EMAIL")
            .ok()
            .filter(|s| !s.is_empty())
        else {
            return Ok(());
        };

        let email = render_email(
            admin_email,
            ADMIN_NOTIFICATION_SUBJECT,
            ADMIN_NOTIFICATION_HTML,
            ADMIN_NOTIFICATION_TEXT,
            &json!({
              "name": user.name,
              "email": user.email,
            }),
        )?;

        Self::dispatch(ctx, email).await
    }
}

#[cfg(test)]
mod tests {
    use super::render;
    use serde_json::json;

    #[test]
    fn renders_locals_into_template() {
        let rendered = render("Welcome {{name}}", &json!({"name": "loco"}), false).unwrap();
        assert_eq!(rendered, "Welcome loco");
    }

    #[test]
    fn escapes_html_when_autoescape_enabled() {
        let rendered = render("<p>{{name}}</p>", &json!({"name": "<script>"}), true).unwrap();
        assert_eq!(rendered, "<p>&lt;script&gt;</p>");
    }
}
