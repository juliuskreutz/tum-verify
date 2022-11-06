use anyhow::Result;
use lettre::{message::MultiPart, transport::smtp::authentication, AsyncTransport};

use crate::{config::EmailConfig, prelude::Language};

const EMAIL_HTML: &str = include_str!("email.html");
const EMAIL_TXT: &str = include_str!("email.txt");

pub struct EmailData {
    language: Language,
    tum_id: String,
    token: uuid::Uuid,
    user_tag: String,
    guild_icon_url: String,
    guild_name: String,
}

impl EmailData {
    pub fn new(
        language: Language,
        tum_id: String,
        token: uuid::Uuid,
        user_tag: String,
        guild_icon_url: String,
        guild_name: String,
    ) -> Self {
        Self {
            language,
            tum_id,
            token,
            user_tag,
            guild_icon_url,
            guild_name,
        }
    }
}

pub async fn send_email(email_config: &EmailConfig, email_data: EmailData) -> Result<()> {
    let language = email_data.language;

    let email_html = EMAIL_HTML
        .replace("{{token}}", &email_data.token.to_string())
        .replace("{{user_tag}}", &email_data.user_tag)
        .replace("{{guild_icon_url}}", &email_data.guild_icon_url)
        .replace("{{guild_name}}", &email_data.guild_name)
        .replace("{{title_email}}", &language.title_email())
        .replace("{{text_email}}", &language.text_email())
        .replace("{{text_email_user}}", &language.text_email_user());

    let email_txt = EMAIL_TXT
        .replace("{{token}}", &email_data.token.to_string())
        .replace("{{user_tag}}", &email_data.user_tag)
        .replace("{{guild_name}}", &email_data.guild_name)
        .replace("{{title_email}}", &language.title_email())
        .replace("{{text_email}}", &language.text_email())
        .replace("{{text_email_user}}", &language.text_email_user())
        .replace("{{text_email_token}}", &language.text_email_token());

    let email = lettre::Message::builder()
        .from(format!("{} <{}>", email_config.name(), email_config.email()).parse()?)
        .to(format!("<{}@mytum.de>", email_data.tum_id).parse()?)
        .subject(language.title_email())
        .multipart(MultiPart::alternative_plain_html(email_txt, email_html))?;

    let creds = authentication::Credentials::new(
        email_config.user().clone(),
        email_config.password().clone(),
    );

    let mailer =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay("cepheus.uberspace.de")?
            .credentials(creds)
            .build();

    mailer.send(email).await?;

    Ok(())
}
