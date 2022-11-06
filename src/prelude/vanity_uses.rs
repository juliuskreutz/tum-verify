use crate::serenity;
use anyhow::{Context, Result};

#[serenity::async_trait]
pub trait VanityUses {
    async fn vanity_uses(&self, ctx: &serenity::Context) -> Result<u64>;
}

#[serenity::async_trait]
impl VanityUses for serenity::GuildId {
    async fn vanity_uses(&self, ctx: &serenity::Context) -> Result<u64> {
        let vanity_url = self.vanity_url(&ctx).await?;

        self.invites(&ctx)
            .await?
            .iter()
            .find(|i| i.url() == vanity_url)
            .map(|i| i.uses)
            .context("No vanity url")
    }
}
