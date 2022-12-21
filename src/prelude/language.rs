use crate::serenity;
use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Sequence)]
pub enum Language {
    German,
    English,
    Russian,
}

impl Language {
    pub fn init_message(guild: &serenity::PartialGuild) -> String {
        format!("
            :flag_de: Willkommen an den **{0}** Server! Wenn Sie die Verifikationsanleitungen auf **Deutsch** haben wollen, drücken Sie unter der Nachricht auf die Deutsche Flagge.

            :flag_gb: Welcome to the **{0}** server! If you want to proceed with the **English** verification instructions, please press the button of the United Kingdom below.

            :flag_ru: Добро пожаловать на сервер «{0}». Чтобы пройти верификацию с _русской_ версией инструкций, нажмите на флаг России снизу.
        ", &guild.name)
    }

    pub fn emoji(&self) -> serenity::ReactionType {
        match self {
            Language::German => serenity::ReactionType::Unicode("🇩🇪".to_string()),
            Language::English => serenity::ReactionType::Unicode("🇬🇧".to_string()),
            Language::Russian => serenity::ReactionType::Unicode("🇷🇺".to_string())
        }
    }

    pub fn btn_enter(&self) -> String {
        match self {
            Language::German => "Eingeben".to_string(),
            Language::English => "Enter".to_string(),
            Language::Russian => "Ввод".to_string(),    
        }
    }

    pub fn btn_abort(&self) -> String {
        match self {
            Language::German => "Abbrechen".to_string(),
            Language::English => "Abort".to_string(),
            Language::Russian => "Отменить".to_string(),
        }
    }

    pub fn text_tum_id(&self) -> String {
        match self {
            Language::German => "TUM Kennung".to_string(),
            Language::English => "TUM Id".to_string(),
            Language::Russian => "ТУМ Логин".to_string(),
        }
    }

    pub fn text_click_btn_for_tum_id(&self) -> String {
        match self {
            Language::German => "Um mit der Verifikation zu starten, bitte drücken Sie auf den Knopf unten und geben Sie Ihre TUM Id ein.".to_string(),
            Language::English => "To start with the verification process, please click the button below and provide your tum id.".to_string(),
            Language::Russian => "Чтобы начать верификацию нажмите кнопку снизу и введите ваш ТУМ-овый код.".to_string(),
        }
    }

    pub fn text_click_btn_for_tum_id_err(&self) -> String {
        let r = format!("{}\n\n", self.text_click_btn_for_tum_id());
        match self {
            Language::German => r + ":warning: Falsches Format der TUM Kennung (ge69tum)",
            Language::English => r + ":warning: Wrong TUM Id format (ge69tum)",
            Language::Russian => r + ":warning: Неверный формат идентификации ТУМа (ge69tum)",
        }
    }

    pub fn text_email_sent(&self, tum_id: &str) -> String {
        match self {
            Language::German => format!(
                "
                Eine Email wurde an Ihre TUM-Email geschickt. Bitte schreiben Sie den dort angegebenen code hier rein \
                ([TUM-Email Anleitung](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'TUM-Email Anleitung')). \
                Email verschickt an:
                ```\n{tum_id}@mytum.de\n```
                "
            ),
            Language::English => format!(
                "
                An email has been sent to your TUM-account. Please send the code contained within into this channel \
                ([TUM-Email Guide](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'How to access your TUM-Email')). \
                Email sent to:
                ```\n{tum_id}@mytum.de\n```
                "
            ),

            Language::Russian => format!(
                "
                На ваш аккаунт ТУМа был выслан э-майл. Пожалуйста введите код в этот канал. \
                ([TUM-Email Guide](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'How to access your TUM-Email')). \
                Код выслан на:
                ```\n{tum_id}@mytum.de\n```
                "
            ),
        }
    }

    pub fn text_token_err(&self, tum_id: &str) -> String {
        let r = format!("{}\n", self.text_email_sent(tum_id));
        match self {
            Language::German => r + ":warning: Ungültiger token eingegeben. Bitte geben Sie den Token ein, den Sie in der email gekriegt haben.",
            Language::English => r + ":warning: Invalid token entered. Please provide the token, which you created in your email.",
            Language::Russian => r + ":warning: Неверный код. Пожалуйста введите код, который был в вашем е мейле.",            
        }
    }

    pub fn text_success(&self) -> String {
        match self {
            Language::German => {
                "Sie sind jetzt verifiziert und haben Zugang zu Servern des TUM Netzwerks."
                    .to_string()
            }
            Language::English => {
                "You are now verified and have access to servers within the TUM Network."
                    .to_string()
            }
            Language::Russian => {
                "Верификация окончена и у вас теперь есть доступ к серверам ТУМа."
                    .to_string()
            }
        }
    }

    pub fn text_token(&self) -> String {
        "Token".to_string()
    }

    pub fn title_network_verification(&self) -> String {
        match self {
            Language::German => "(Studentenorganisierter) TUM Discord ○ Verifikation".to_string(),
            Language::English => "(Student-run) TUM Discord ○ Verification".to_string(),
            Language::Russian => "Студенческий Дискорд ТУМа ○ Верификация".to_string(),
        }
    }

    pub fn title_email(&self) -> String {
        match self {
            Language::German => "TUM Discord Netzwerk ○ Verifikations Code".to_string(),
            Language::English => "TUM Discord Network ○ Verification Code".to_string(),
            Language::Russian => "Студенческий Дискорд ТУМа ○ Верификационный код".to_string(),
        }
    }

    pub fn text_email(&self) -> String {
        match self {
            Language::German => {
                "Jemand hat deine TUM-Kennung verwendet, um auf dein Studenten-Netzwerk im Discord zuzugreifen. Wenn das nicht Sie waren, bitte ignorieren Sie die email."
                    .to_string()
            }
            Language::English => {
                "Someone used your TUM Id to sign up to the student-run TUM Discord Network. If this was not you, please ignore this email.".to_string()
            }
            Language::Russian => {
                "Кто-то вошёл в студенческий дискорд сервер с вашем ТУМ Id. Если это были не вы то проигнорируйте это сообщение.".to_string()
            }
        }
    }

    pub fn text_email_user(&self) -> String {
        match self {
            Language::German => "Nutzer".to_string(),
            Language::English => "User".to_string(),
            Language::Russian => "Пользователь".to_string(),
        }
    }

    pub fn text_email_token(&self) -> String {
        match self {
            Language::German => "UNTEN FINDEN SIE DEN TOKEN ANGEHÄNGT AN DIE EMAIL".to_string(),
            Language::English => "BELOW YOU CAN FIND THE TOKEN ATTACHED TO THE EMAIL".to_string(),
            Language::Russian => "ВНИЗУ В Е-МАЙЛЕ ВЫ НАЙДЁТЕ КОД".to_string(),
        }
    }
}
