use std::collections;

use anyhow::Result;
use config::{Config, GuildConfig};
use prelude::*;
use ui::message;

mod config;
mod database;
mod email;
mod prelude;
mod serenity;
mod ui;
mod util;
mod verify;

const APPLICATION_ID_INIT: &str = "init";
const OPTION_ID_ADD: &str = "add";
const OPTION_ID_REMOVE: &str = "remove";
const OPTION_ID_LOG: &str = "log";

pub struct Data {
    pub config: Config,
    pub vanity_uses: serenity::Mutex<collections::HashMap<serenity::GuildId, u64>>,
}

struct Handler {
    data: Data,
}

impl Handler {
    async fn ready(&self, ctx: &serenity::Context, ready: &serenity::Ready) -> Result<()> {
        let mut vanity_uses_lock = self.data.vanity_uses.lock().await;

        for guild in &ready.guilds {
            guild
                .id
                .set_application_commands(&ctx, |b| {
                    b.create_application_command(|c| {
                        c.name(APPLICATION_ID_INIT)
                            .description("Inits the message and bot")
                            .create_option(|o| {
                                o.name(OPTION_ID_ADD)
                                    .description(OPTION_ID_ADD)
                                    .kind(serenity::CommandOptionType::Role)
                                    .required(false)
                            })
                            .create_option(|o| {
                                o.name(OPTION_ID_REMOVE)
                                    .description(OPTION_ID_REMOVE)
                                    .kind(serenity::CommandOptionType::Role)
                                    .required(false)
                            })
                            .create_option(|o| {
                                o.name(OPTION_ID_LOG)
                                    .description(OPTION_ID_LOG)
                                    .kind(serenity::CommandOptionType::Channel)
                                    .required(false)
                            })
                    })
                })
                .await?;

            if let Ok(uses) = guild.id.vanity_uses(ctx).await {
                vanity_uses_lock.insert(guild.id, uses);
            }
        }

        Ok(())
    }
}

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn ready(&self, ctx: serenity::Context, ready: serenity::Ready) {
        self.ready(&ctx, &ready).await.unwrap();
    }

    async fn guild_member_addition(&self, ctx: serenity::Context, mut member: serenity::Member) {
        verify::join(&ctx, &self.data, &mut member).await.unwrap();
    }

    async fn interaction_create(&self, ctx: serenity::Context, interaction: serenity::Interaction) {
        match interaction {
            serenity::Interaction::ApplicationCommand(interaction) => {
                if interaction.data.name == APPLICATION_ID_INIT {
                    let guild_id = interaction.guild_id.unwrap();

                    let add = interaction
                        .data
                        .options
                        .iter()
                        .find(|o| o.name == OPTION_ID_ADD)
                        .and_then(|o| o.value.clone())
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .and_then(|r| r.parse().ok());

                    let remove = interaction
                        .data
                        .options
                        .iter()
                        .find(|o| o.name == OPTION_ID_REMOVE)
                        .and_then(|o| o.value.clone())
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .and_then(|r| r.parse().ok());

                    let log = interaction
                        .data
                        .options
                        .iter()
                        .find(|o| o.name == OPTION_ID_LOG)
                        .and_then(|o| o.value.clone())
                        .and_then(|v| v.as_str().map(|s| s.to_string()))
                        .and_then(|r| r.parse().ok());

                    GuildConfig::new(add, remove, log).write(guild_id).unwrap();

                    let guild = serenity::Guild::get(&ctx, guild_id).await.unwrap();

                    interaction
                        .create_interaction_response(&ctx, |r| {
                            r.interaction_response_data(|m| {
                                message::verify(m, &guild.icon_url().unwrap())
                            })
                        })
                        .await
                        .unwrap();
                }
            }
            serenity::Interaction::MessageComponent(interaction) => {
                if let Ok(language) = serde_json::from_str::<Language>(&interaction.data.custom_id)
                {
                    if let Err(e) = verify::verify(&ctx, language, &self.data, &interaction).await {
                        if let Ok(Some(log)) =
                            GuildConfig::read(interaction.guild_id.unwrap()).map(|gc| *gc.log())
                        {
                            log.say(&ctx, format!("{e}")).await.unwrap();
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    database::init()?;

    let config = Config::read()?;

    serenity::Client::builder(config.token(), serenity::GatewayIntents::non_privileged())
        .event_handler(Handler {
            data: Data {
                config,
                vanity_uses: serenity::Mutex::new(collections::HashMap::new()),
            },
        })
        .await?
        .start()
        .await?;

    Ok(())
}
