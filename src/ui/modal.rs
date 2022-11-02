use crate::{serenity, Language, ModalInteractionId, INPUT_ID_TOKEN, INPUT_ID_TUM_ID};

pub fn tum_id<'a, 'b>(
    response: &'b mut serenity::CreateInteractionResponse<'a>,
    language: Language,
) -> &'b mut serenity::CreateInteractionResponse<'a> {
    response
        .kind(serenity::InteractionResponseType::Modal)
        .interaction_response_data(|d| {
            d.custom_id(serde_json::to_string(&ModalInteractionId::EnteredTumId).unwrap())
                .title(language.text_tum_id())
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_input_text(|i| {
                            i.custom_id(INPUT_ID_TUM_ID)
                                .label(language.text_tum_id())
                                .placeholder("ab12cde")
                                .style(serenity::component::InputTextStyle::Short)
                        })
                    })
                })
        })
}

pub fn token<'a, 'b>(
    response: &'b mut serenity::CreateInteractionResponse<'a>,
    language: Language,
) -> &'b mut serenity::CreateInteractionResponse<'a> {
    response
        .kind(serenity::InteractionResponseType::Modal)
        .interaction_response_data(|d| {
            d.custom_id(serde_json::to_string(&ModalInteractionId::EnteredUuid).unwrap())
                .title(language.text_token())
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_input_text(|i| {
                            i.custom_id(INPUT_ID_TOKEN)
                                .label(language.text_token())
                                .placeholder("61ddb5d4-043f-4f6e-9b0e-382aa67d7827")
                                .style(serenity::component::InputTextStyle::Short)
                        })
                    })
                })
        })
}
