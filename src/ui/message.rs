use super::embed;
use crate::{serenity, Language, MessageBuilder, MessageInteractionId};

pub fn verify<'a, T: MessageBuilder>(m: &'a mut T, guild: &serenity::PartialGuild) -> &'a mut T {
    m.embed(|e| embed::verify(e, guild)).components(|c| {
        c.create_action_row(|r| {
            for l in enum_iterator::all::<Language>() {
                r.create_button(|b| {
                    b.style(serenity::component::ButtonStyle::Secondary)
                        .emoji(l.emoji())
                        .custom_id(serde_json::to_string(&l).unwrap())
                });
            }

            r
        })
    })
}

pub fn tum_id<T: MessageBuilder>(m: &mut T, language: Language) -> &mut T {
    m.embed(|e| embed::tum_id(e, language)).components(|c| {
        c.create_action_row(|r| {
            r.create_button(|button| {
                button
                    .label(language.btn_enter())
                    .custom_id(serde_json::to_string(&MessageInteractionId::EnterTumId).unwrap())
            });

            menu(r, language)
        })
    })
}

pub fn invalid_tum_id<T: MessageBuilder>(m: &mut T, language: Language) -> &mut T {
    m.embed(|e| embed::invalid_tum_id(e, language))
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|button| {
                    button.label(language.btn_enter()).custom_id(
                        serde_json::to_string(&MessageInteractionId::EnterTumId).unwrap(),
                    )
                });

                menu(r, language)
            })
        })
}

pub fn token<'a, T: MessageBuilder>(m: &'a mut T, language: Language, tum_id: &str) -> &'a mut T {
    m.embed(|e| embed::token(e, language, tum_id))
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|button| {
                    button.label(language.btn_enter()).custom_id(
                        serde_json::to_string(&MessageInteractionId::EnterToken).unwrap(),
                    )
                });

                menu(r, language)
            })
        })
}

pub fn invalid_token<'a, T: MessageBuilder>(
    m: &'a mut T,
    language: Language,
    tum_id: &str,
) -> &'a mut T {
    m.embed(|e| embed::invalid_token(e, language, tum_id))
        .components(|c| {
            c.create_action_row(|r| {
                r.create_button(|button| {
                    button.label(language.btn_enter()).custom_id(
                        serde_json::to_string(&MessageInteractionId::EnterToken).unwrap(),
                    )
                });

                menu(r, language)
            })
        })
}

pub fn success<T: MessageBuilder>(m: &mut T, language: Language) -> &mut T {
    m.embed(|e| embed::success(e, language)).components(|c| c)
}

fn menu(row: &mut serenity::CreateActionRow, language: Language) -> &mut serenity::CreateActionRow {
    for l in enum_iterator::all::<Language>() {
        row.create_button(|b| {
            b.style(serenity::component::ButtonStyle::Secondary)
                .emoji(l.emoji())
                .custom_id(serde_json::to_string(&MessageInteractionId::Language(l)).unwrap());

            if language == l {
                b.disabled(true);
            }

            b
        });
    }

    row.create_button(|button| {
        button
            .style(serenity::component::ButtonStyle::Danger)
            .label(language.btn_abort())
            .custom_id(serde_json::to_string(&MessageInteractionId::Terminate).unwrap())
    })
}
