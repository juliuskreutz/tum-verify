use std::{sync, time};

use crate::{
    config::GuildConfig,
    database, email,
    serenity::{self, Mentionable},
    INPUT_ID_TOKEN, INPUT_ID_TUM_ID,
};
use anyhow::{Context, Result};
use derive_getters::Getters;

use crate::{
    ui::{message, modal},
    Data, InteractionHandle, Language, MessageInteractionId, ModalInteractionId, VanityUses,
};

#[derive(Getters)]
struct Info {
    tum_id: String,
    token: uuid::Uuid,
}

impl Default for Info {
    fn default() -> Self {
        Self {
            tum_id: "ab12cde".to_string(),
            token: uuid::Uuid::new_v4(),
        }
    }
}

struct State {
    language: Language,
    message_state: MessageState,
    interaction_state: InteractionState,
}

impl State {
    fn new(language: Language) -> Self {
        Self {
            language,
            message_state: MessageState::TumId,
            interaction_state: InteractionState::None,
        }
    }
}

enum InteractionState {
    AskTumId,
    ReceivedInvalidTumId,
    ReceivedValidTumId,
    AskToken,
    ReceivedInvalidToken,
    ReceivedValidToken,
    Terminated,
    None,
}

enum MessageState {
    TumId,
    InvalidTumId,
    Token,
    InvalidToken,
    Success,
    None,
}

pub async fn join(
    ctx: &serenity::Context,
    data: &Data,
    member: &mut serenity::Member,
) -> Result<()> {
    {
        let mut vanity_uses_lock = data.vanity_uses.lock().await;

        let vanity_uses_before = vanity_uses_lock[&member.guild_id];
        let vanity_uses_after = member.guild_id.vanity_uses(ctx).await?;

        if vanity_uses_after == vanity_uses_before {
            println!("{vanity_uses_after} {vanity_uses_before}");
            return Ok(());
        }

        vanity_uses_lock
            .entry(member.guild_id)
            .and_modify(|uses| *uses += 1);
    }

    if database::is_verified(member.user.id)? {
        let guild_config = GuildConfig::read(member.guild_id)?;

        if let Some(add) = guild_config.add() {
            member.add_role(ctx, add).await?;
        }

        if let Some(remove) = guild_config.remove() {
            member.remove_role(ctx, remove).await?;
        }
    }

    Ok(())
}

pub async fn verify(
    ctx: &serenity::Context,
    language: Language,
    data: &Data,
    interaction: &serenity::MessageComponentInteraction,
) -> Result<()> {
    interaction.defer(&ctx).await?;

    let mut member = interaction.member.clone().context("Not a Member?")?;
    let guild = serenity::Guild::get(ctx, interaction.guild_id.context("No guild?")?).await?;
    let guild_config = GuildConfig::read(guild.id)?;

    let mut state = State::new(language);
    let mut info = Info::default();

    let channel = member.user.create_dm_channel(&ctx).await?;

    let mut message = channel
        .send_message(&ctx, |m| message::tum_id(m, state.language))
        .await?;

    loop {
        let interaction = tokio::select! {
            interaction = message.await_component_interaction(ctx).timeout(time::Duration::from_secs(60 * 5)) => {
                either::Left(interaction)
            }
            interaction = message.await_modal_interaction(ctx).timeout(time::Duration::from_secs(60 * 5)) => {
                either::Right(interaction)
            }
        };

        let handle = match handle_interaction(interaction, &mut state, &mut info) {
            Ok(handle) => handle,
            Err(_) => {
                message.delete(&ctx).await?;
                return Ok(());
            }
        };

        match state.interaction_state {
            InteractionState::AskTumId => {
                handle
                    .respond(&ctx, |r| modal::tum_id(r, state.language))
                    .await?;

                state.message_state = MessageState::None;
            }
            InteractionState::ReceivedInvalidTumId => {
                handle.defer(&ctx).await?;

                state.message_state = MessageState::InvalidTumId;
            }
            InteractionState::ReceivedValidTumId => {
                handle.defer(&ctx).await?;

                //TODO: Verification limit message
                if database::get_verifications(&info.tum_id)? > 4 {
                    message
                        .edit(&ctx, |m| {
                            m.embed(|e| e.description("This TUM Id was used more than 4 times"))
                        })
                        .await?;
                    return Ok(());
                }

                //TODO: Sending email message
                message
                    .edit(&ctx, |m| {
                        m.embed(|e| e.description("Sending email..."))
                            .components(|c| c)
                    })
                    .await?;

                let email_data = email::EmailData::new(
                    state.language,
                    info.tum_id.clone(),
                    info.token,
                    member.user.tag(),
                    guild.icon_url().context("No icon?")?,
                    guild.name.clone(),
                );

                email::send_email(data.config.email(), email_data).await?;

                state.message_state = MessageState::Token;
            }
            InteractionState::AskToken => {
                handle
                    .respond(&ctx, |r| modal::token(r, state.language))
                    .await?;

                state.message_state = MessageState::None;
            }
            InteractionState::ReceivedInvalidToken => {
                handle.defer(&ctx).await?;

                state.message_state = MessageState::InvalidToken;
            }
            InteractionState::ReceivedValidToken => {
                handle.defer(&ctx).await?;

                if let Some(add) = guild_config.add() {
                    member.add_role(ctx, add).await?;
                }

                if let Some(remove) = guild_config.remove() {
                    member.remove_role(ctx, remove).await?;
                }

                database::add_verified(member.user.id, &info.tum_id)?;

                if let Some(log) = guild_config.log() {
                    log.say(ctx, format!("{} verified!", member.user.mention()))
                        .await?;
                }

                state.message_state = MessageState::Success;
            }
            InteractionState::Terminated => {
                message.delete(&ctx).await?;

                return Ok(());
            }
            InteractionState::None => {
                handle.defer(&ctx).await?;
            }
        }

        match state.message_state {
            MessageState::TumId => {
                message
                    .edit(&ctx, |m| message::tum_id(m, state.language))
                    .await?
            }
            MessageState::InvalidTumId => {
                message
                    .edit(&ctx, |m| message::invalid_tum_id(m, state.language))
                    .await?
            }
            MessageState::Token => {
                message
                    .edit(&ctx, |m| message::token(m, state.language, &info.tum_id))
                    .await?
            }
            MessageState::InvalidToken => {
                message
                    .edit(&ctx, |m| {
                        message::invalid_token(m, state.language, &info.tum_id)
                    })
                    .await?
            }
            MessageState::Success => {
                message
                    .edit(&ctx, |m| message::success(m, state.language))
                    .await?
            }
            MessageState::None => {}
        }
    }
}

fn handle_interaction(
    interaction: either::Either<
        Option<sync::Arc<serenity::MessageComponentInteraction>>,
        Option<sync::Arc<serenity::ModalSubmitInteraction>>,
    >,
    state: &mut State,
    info: &mut Info,
) -> Result<InteractionHandle> {
    match interaction {
        either::Left(Some(interaction)) => {
            let interaction_id = serde_json::from_str(&interaction.data.custom_id)?;

            match interaction_id {
                MessageInteractionId::Language(language) => {
                    state.language = language;
                    state.interaction_state = InteractionState::None;
                }
                MessageInteractionId::EnterTumId => {
                    state.interaction_state = InteractionState::AskTumId;
                }
                MessageInteractionId::EnterToken => {
                    state.interaction_state = InteractionState::AskToken;
                }
                MessageInteractionId::Terminate => {
                    state.interaction_state = InteractionState::Terminated
                }
            }

            Ok(InteractionHandle::Message(interaction))
        }
        either::Right(Some(interaction)) => {
            let interaction_id = serde_json::from_str(&interaction.data.custom_id)?;

            let mut inputs = interaction
                .data
                .components
                .iter()
                .flat_map(|r| &r.components)
                .filter_map(|c| match c {
                    serenity::component::ActionRowComponent::InputText(input) => Some(input),
                    _ => None,
                });

            match interaction_id {
                ModalInteractionId::EnteredTumId => {
                    let tum_id = inputs
                        .find(|i| i.custom_id == INPUT_ID_TUM_ID)
                        .context("Input not found")?
                        .value
                        .clone();

                    if crate::is_valid_tum_id(&tum_id) {
                        state.interaction_state = InteractionState::ReceivedValidTumId;
                        info.tum_id = tum_id;
                    } else {
                        state.interaction_state = InteractionState::ReceivedInvalidTumId;
                    }
                }
                ModalInteractionId::EnteredUuid => {
                    if inputs
                        .find(|i| i.custom_id == INPUT_ID_TOKEN)
                        .context("Input not found")?
                        .value
                        .parse::<uuid::Uuid>()
                        .map(|token| token == info.token)
                        .unwrap_or(false)
                    {
                        state.interaction_state = InteractionState::ReceivedValidToken;
                    } else {
                        state.interaction_state = InteractionState::ReceivedInvalidToken;
                    }
                }
            }

            Ok(InteractionHandle::Modal(interaction))
        }
        _ => anyhow::bail!("No interaction"),
    }
}
