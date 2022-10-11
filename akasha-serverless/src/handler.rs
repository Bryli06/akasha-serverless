use crate::utils::{Error, InteractionError};
use crate::discord::*;


impl Interaction {
    pub fn get_data(&self) -> Result<&InteractionData, Error> {
        self.data
            .as_ref()
            .ok_or_else(|| Error::PayloadError("No data".to_string()))
    }

    pub fn ping_handler(&self) -> InteractionResponse {
        InteractionResponse {
            interaction_callback_type: InteractionCallbackType::Pong,
            data: None,
        }
    }

    pub async fn command_handler(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, InteractionError> {
        let interaction_data = self.get_data().map_err(|_| InteractionError::Error("No Data".to_string()))?;
        match interaction_data {
            InteractionData::ApplicationCommand(data) => {
                Err(InteractionError::CommandNotFound(data.name.clone()))
            },
            _ => Err(InteractionError::Error("Interaction was not a command".to_string()))
        }
    }

    pub async fn handle_interaction(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, Error> {
        match self.interaction_type {
            _ => Err(Error::PayloadError("Interaction not implemented".to_string())),
        }
    }
}
