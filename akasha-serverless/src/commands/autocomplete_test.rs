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
        unimplemented!()
    }

    async fn autocomplete(&self, input: &Input) -> Result<Option<AutocompleteInteractionCallbackData>, InteractionError> {
        unimplemented!()
    }

    async fn respond(&self, input: &Input) -> Result<MessagesInteractionCallbackData, InteractionError> {
        unimplemented!()
    }
}
