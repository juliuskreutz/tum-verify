use crate::{serenity, Language};

pub fn verify<'a>(
    embed: &'a mut ::serenity::builder::CreateEmbed,
    guild: &serenity::PartialGuild,
) -> &'a mut ::serenity::builder::CreateEmbed {
    embed
        .thumbnail(guild.icon_url().unwrap())
        .title(&guild.name)
        .description(format!("
            :flag_de: Willkommen an den **{0}** Server! Wenn Sie die Verifikationsanleitungen auf **Deutsch** haben wollen, drücken Sie unter der Nachricht auf die Deutsche Flagge.

            :flag_gb: Welcome to the **{0}** server! If you want to proceed with the **English** verification instructions, please press the button of the United Kingdom below.
        ", &guild.name))
}

//TODO: Make this nice
pub fn tum_id(embed: &mut serenity::CreateEmbed, language: Language) -> &mut serenity::CreateEmbed {
    embed
        .color((52, 137, 235))
        .title(language.title_network_verification());

    match language {
        Language::German => embed.description(
            "
                Hallo bitte button drücken um TUM Kennung einzugeben
                ",
        ),
        Language::English => embed.description(
            "
                Hello please press button to enter TUM Id
                ",
        ),
    }
}

//TODO: Make this nice
pub fn invalid_tum_id(
    embed: &mut serenity::CreateEmbed,
    language: Language,
) -> &mut serenity::CreateEmbed {
    embed
        .color((242, 121, 80))
        .title(language.title_network_verification());

    match language {
        Language::German => embed.description(
            "
            Hallo bitte button drücken um TUM Kennung einzugeben

            :warning: Falsches Format der TUM Kennung (ab12cde)
            ",
        ),
        Language::English => embed.description(
            "
            Hello please press button to enter TUM Id

            :warning: Wrong TUM Id format (ab12cde)
            ",
        ),
    }
}

pub fn token<'a>(
    embed: &'a mut serenity::CreateEmbed,
    language: Language,
    tum_id: &str,
) -> &'a mut serenity::CreateEmbed {
    embed
        .color((52, 137, 235))
        .title(language.title_network_verification());

    match language{
        Language::German => embed
            .description(format!(
                "
                Eine Email wurde an Ihre TUM-Email geschickt. Bitte schreiben Sie den dort angegebenen code hier rein \
                ([TUM-Email Anleitung](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'TUM-Email Anleitung')). \
                Email verschickt an:
                ```\n{tum_id}@mytum.de\n```
                "
            )),
        Language::English => embed
            .description(format!(
                "
                An email has been sent to your TUM-account. Please send the code contained within into this channel \
                ([TUM-Email Guide](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'How to access your TUM-Email')). \
                Email sent to:
                ```\n{tum_id}@mytum.de\n```
                "
            )),
    }
}

pub fn invalid_token<'a>(
    embed: &'a mut serenity::CreateEmbed,
    language: Language,
    tum_id: &str,
) -> &'a mut serenity::CreateEmbed {
    embed
        .color((242, 121, 80))
        .title(language.title_network_verification());

    match language {
        Language::German => embed
            .description(format!(
                "
                Eine Email wurde an Ihre TUM-Email geschickt. Bitte schreiben Sie den dort angegebenen code hier rein \
                ([TUM-Email Anleitung](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'TUM-Email Anleitung')). \
                Email verschickt an:
                ```\n{tum_id}@mytum.de\n```
                :warning: Ungültiger token eingegeben. Bitte geben Sie den Token ein, den Sie in der email gekriegt haben.
                "
            )),
        Language::English => embed
            .description(format!(
                "
                An email has been sent to your TUM-account. Please send the code contained within into this channel \
                ([TUM-Email Guide](https://campus.tum.de/tumonline/ee/ui/ca2/app/desktop/#/pl/ui/$ctx/help.file_help?$ctx=design=ca2;header=max;lang=de&app_kb=BM&corg=&seite_nr=500231&sprache_nr=1 'How to access your TUM-Email')). \
                Email sent to:
                ```\n{tum_id}@mytum.de\n```
                :warning: Invalid token entered. Please provide the token, which you created in your email.
                "
            )),
    }
}

pub fn success(
    embed: &mut serenity::CreateEmbed,
    language: Language,
) -> &mut serenity::CreateEmbed {
    embed
        .color((52, 235, 82))
        .title(language.title_network_verification());

    match language {
        Language::German => embed.description(
            "Sie sind jetzt verifiziert und haben Zugang zu Servern des TUM Netzwerks.",
        ),
        Language::English => embed
            .description("You are now verified and have access to servers within the TUM Network."),
    }
}
