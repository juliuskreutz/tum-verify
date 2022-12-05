use crate::{serenity, Language};

pub fn verify<'a>(
    embed: &'a mut ::serenity::builder::CreateEmbed,
    guild: &serenity::PartialGuild,
) -> &'a mut ::serenity::builder::CreateEmbed {
    embed
        .thumbnail(guild.icon_url().unwrap())
        .title(&guild.name)
        .description(Language::init_message(guild))
}

pub fn tum_id(embed: &mut serenity::CreateEmbed, language: Language) -> &mut serenity::CreateEmbed {
    embed
        .color((52, 137, 235))
        .title(language.title_network_verification())
        .description(language.text_click_btn_for_tum_id())
}

pub fn invalid_tum_id(
    embed: &mut serenity::CreateEmbed,
    language: Language,
) -> &mut serenity::CreateEmbed {
    embed
        .color((242, 121, 80))
        .title(language.title_network_verification())
        .description(language.text_click_btn_for_tum_id_err())
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
