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
        .title(language.title_network_verification())
        .description(language.text_email_sent(tum_id))
}

pub fn invalid_token<'a>(
    embed: &'a mut serenity::CreateEmbed,
    language: Language,
    tum_id: &str,
) -> &'a mut serenity::CreateEmbed {
    embed
        .color((242, 121, 80))
        .title(language.title_network_verification())
        .description(language.text_token_err(tum_id))
}

pub fn success(
    embed: &mut serenity::CreateEmbed,
    language: Language,
) -> &mut serenity::CreateEmbed {
    embed
        .color((52, 235, 82))
        .title(language.title_network_verification())
        .description(language.text_success())
}
