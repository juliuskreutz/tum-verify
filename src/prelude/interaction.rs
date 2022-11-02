use std::sync;

use crate::{serenity, Language};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub const BUTTON_ID_VERIFY: &str = "verify";

pub const INPUT_ID_TUM_ID: &str = "tum_id";
pub const INPUT_ID_TOKEN: &str = "token";

#[derive(Serialize, Deserialize)]
pub enum MessageInteractionId {
    Language(Language),
    EnterTumId,
    EnterToken,
    Terminate,
}

#[derive(Serialize, Deserialize)]
pub enum ModalInteractionId {
    EnteredTumId,
    EnteredUuid,
}

pub enum InteractionHandle {
    Message(sync::Arc<serenity::MessageComponentInteraction>),
    Modal(sync::Arc<serenity::ModalSubmitInteraction>),
}

impl InteractionHandle {
    pub async fn defer(&self, http: impl AsRef<serenity::Http>) -> Result<()> {
        match self {
            Self::Message(interaction) => interaction.defer(http).await?,
            Self::Modal(interaction) => interaction.defer(http).await?,
        }

        Ok(())
    }

    pub async fn respond<'a, F>(&self, http: impl AsRef<serenity::Http>, f: F) -> Result<()>
    where
        for<'b> F: FnOnce(
            &'b mut serenity::CreateInteractionResponse<'a>,
        ) -> &'b mut serenity::CreateInteractionResponse<'a>,
    {
        match self {
            Self::Message(interaction) => interaction.create_interaction_response(http, f).await?,
            Self::Modal(interaction) => interaction.create_interaction_response(http, f).await?,
        }

        Ok(())
    }
}
