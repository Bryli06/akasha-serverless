use super::{Command, Input};
use async_trait::async_trait;
use crate::discord::*;
use crate::utils::InteractionError;

pub struct About {}

#[async_trait(?Send)]
impl Command for About {
    fn name(&self) -> String {
        "about".to_string() 
    }

    fn description(&self) -> String {
        "How does this bot work".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        None
    }

    async fn autocomplete(&self, input: &Input) -> Result<Option<AutocompleteInteractionCallbackData>, InteractionError> {
        Ok(None)
    }

    async fn respond(&self, input: &Input) -> Result<MessagesInteractionCallbackData, InteractionError> {
        let embed = Embed {
            title: Some("About this bot".to_string()),
            embed_type: Some(EmbedType::Rich),
            description: Some("This bot made by bryanli#2718 is the TC bot for [Kusanali mains](https://discord.gg/kusanali). You can invite the bot at <https://kusanalimains.com/invite/>. The source code is all open source and can be found [here](https://github.com/Bryli06/akasha-serverless). As this was designed to be a serverless discord bot, it is hosted for free on cloudflare workers!".to_string()),
            url: None,
            color: Some(0x198754),
            footer: None,
            image: None,
            thumbnail: None, 
            fields: None,
        };

        Ok(MessagesInteractionCallbackData {
            content: None,
            components: None,
            embeds: Some(vec![embed]),
            attachments: None,
            flags: None,
        })
    }
}
