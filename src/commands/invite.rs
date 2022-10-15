use super::{Command, Input};
use async_trait::async_trait;
use crate::discord::*;
use crate::utils::InteractionError;

pub struct Invite{}

#[async_trait(?Send)]
impl Command for Invite{
    fn name(&self) -> String {
        "invite".to_string() 
    }

    fn description(&self) -> String {
        "Get invite link".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        None
    }

    async fn autocomplete(&self, input: &Input) -> Result<Option<AutocompleteInteractionCallbackData>, InteractionError> {
        Ok(None)
    }

    async fn respond(&self, input: &Input) -> Result<MessagesInteractionCallbackData, InteractionError> {
        Ok(MessagesInteractionCallbackData {
            content: Some("You can invite Akasha at <https://kusanalimains.com/invite/>.".to_string()),
            components: None,
            embeds: None,
            attachment: None,
        })
    }
}
