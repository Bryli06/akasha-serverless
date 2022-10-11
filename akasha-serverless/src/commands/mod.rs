use async_trait::async_trait;

use crate::discord::*;

use crate::utils::InteractionError;


#[async_trait(?Send)]
pub trait Command {
    fn name(&self) -> String {
       unimplemented!() 
    }

    fn description(&self) -> String {
        unimplemented!()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        unimplemented!()
    }

    async fn autocomplete(&self, input: &Input) -> Result<Option<AutocompleteInteractionCallbackData>, InteractionError> {
        unimplemented!()
    }
}

pub struct Input<'T> {
    pub ctx: &'T mut worker::RouteContext<()>,
}
