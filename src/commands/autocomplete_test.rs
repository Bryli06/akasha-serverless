use super::{Command, Input};
use async_trait::async_trait;
use worker::console_log;
use crate::discord::*;
use crate::utils::InteractionError;

pub struct AutocompleteTest {}

#[async_trait(?Send)]
impl Command for AutocompleteTest {
    fn name(&self) -> String {
        "autocomplete".to_string() 
    }

    fn description(&self) -> String {
        "autocomplete test".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![
            ApplicationCommandOption {
                name: "deez".to_string(),
                autocomplete: Some(true),
                description: "nuts".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::String,
                choices: None,  
                min_value: None,
                max_value: None,
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "pppopoo".to_string(),
                autocomplete: Some(false),
                description: "nutsos".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::String,
                choices: Some(vec![
                    ApplicationCommandOptionChoice{
                        name: "x".to_string(),
                        value: "a".to_string(),
                    },
                    ApplicationCommandOptionChoice{
                        name: "y".to_string(),
                        value: "b".to_string(),
                    },
                    ApplicationCommandOptionChoice{
                        name: "z".to_string(),
                        value: "c".to_string(),
                    },]
                ),  
                min_value: None,
                max_value: None,
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            }
        ])
    }

    async fn autocomplete(&self, input: &Input) -> Result<Option<AutocompleteInteractionCallbackData>, InteractionError> {
        Ok(Some(AutocompleteInteractionCallbackData { 
            choices: (vec![
                ApplicationCommandOptionChoice{
                    name: "a".to_string(),
                    value: "a".to_string(),
                },
                ApplicationCommandOptionChoice{
                    name: "b".to_string(),
                    value: "b".to_string(),
                },
                ApplicationCommandOptionChoice{
                    name: "c".to_string(),
                    value: "c".to_string(),
                },
            ])
        }))
    }

    async fn respond(&self, input: &Input) -> Result<MessagesInteractionCallbackData, InteractionError> {
        let embed = Embed {
            title: Some("Hello".to_string()),
            embed_type: Some(EmbedType::Rich),
            description: Some(format!("deez: {}\npppopoo: {}", input.get_options("deez").unwrap_or("Error"), input.get_options("pppopoo").unwrap_or("Error"))),
            url: None,
            color: Some(0x696969),
            footer: None,
            image: None,
            thumbnail: None, 
            fields: Some(vec![
                EmbedField {
                    name: "Hello".to_string(),
                    value: "e".to_string(),
                    inline: Some(true),
                }
            ]),
        };
        Ok(MessagesInteractionCallbackData {
            content: Some(format!("Hello {}", input.get_options("name").unwrap_or("pp"))),
            components: None,
            embeds: Some(vec![embed]),
            attachment: None,
        })
    }
}
