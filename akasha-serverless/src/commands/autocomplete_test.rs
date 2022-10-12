use super::{Command, Input};
use async_trait::async_trait;
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
                required: Some(false),
                option_type: ApplicationCommandOptionType::String,
                choices: None,  
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
        Ok(MessagesInteractionCallbackData {
            content: Some(format!("Hello {}", input.get_options("name").unwrap_or("pp"))),
            components: None,
            embeds: None,
            attachment: None,
        })
    }
}
