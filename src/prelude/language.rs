use crate::serenity;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Sequence)]
pub enum Language {
    German,
    English,
}

impl Language {
    pub fn emoji(&self) -> serenity::ReactionType {
        match self {
            Language::German => serenity::ReactionType::Unicode("🇩🇪".to_string()),
            Language::English => serenity::ReactionType::Unicode("🇬🇧".to_string()),
        }
    }

    pub fn btn_enter(&self) -> String {
        match self {
            Language::German => "Eingeben".to_string(),
            Language::English => "Enter".to_string(),
        }
    }

    pub fn btn_abort(&self) -> String {
        match self {
            Language::German => "Abbrechen".to_string(),
            Language::English => "Abort".to_string(),
        }
    }

    pub fn text_tum_id(&self) -> String {
        match self {
            Language::German => "TUM Kennung".to_string(),
            Language::English => "TUM Id".to_string(),
        }
    }

    pub fn text_token(&self) -> String {
        "Token".to_string()
    }

    pub fn title_network_verification(&self) -> String {
        match self {
            Language::German => "(Studentenorganisierter) TUM Discord ○ Verifikation".to_string(),
            Language::English => "(Student-run) TUM Discord ○ Verification".to_string(),
        }
    }

    pub fn title_email(&self) -> String {
        match self {
            Language::German => "TUM Discord Netzwerk ○ Verifikations Code".to_string(),
            Language::English => "TUM Discord Network ○ Verification Code".to_string(),
        }
    }

    pub fn text_email(&self) -> String {
        match self {
            Language::German => {
                "Jemand hat deine Kennung benutzt tralalala, Wenn's nicht du bist, ignorieren."
                    .to_string()
            }
            Language::English => {
                "Someone used your TUM Id to etc., if this is not you, ignore.".to_string()
            }
        }
    }

    pub fn text_email_user(&self) -> String {
        match self {
            Language::German => "Nutzer".to_string(),
            Language::English => "User".to_string(),
        }
    }

    pub fn text_email_token(&self) -> String {
        match self {
            Language::German => "UNTEN FINDEN SIE DEN TOKEN ANGEHÄNGT AN DIE EMAIL".to_string(),
            Language::English => "BELOW YOU CAN FIND THE TOKEN ATTACHED TO THE EMAIL".to_string(),
        }
    }
}
