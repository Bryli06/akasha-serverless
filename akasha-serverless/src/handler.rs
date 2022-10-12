use crate::commands::{get_commands, Input};
use crate::utils::{Error, InteractionError};
use crate::discord::*;


impl Interaction {
    fn get_data(&self) -> Result<&InteractionData, Error> {
        self.data
            .as_ref()
            .ok_or_else(|| Error::PayloadError("No data".to_string()))
    }

    fn get_type(&self) -> &InteractionType {
        &self.interaction_type
    }

    fn ping_handler(&self) -> InteractionResponse {
        InteractionResponse {
            interaction_callback_type: InteractionCallbackType::Pong,
            data: None,
        }
    }

    async fn command_handler(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, InteractionError> {
        let data = self.get_data().map_err(|_| InteractionError::Error("No Data".to_string()))?;
        match self.get_type() {
            InteractionType::ApplicationCommand => {
                let commands = get_commands();
                let input = Input {
                    ctx,
                    member: self.member.clone(),
                    user: self.user.clone(),
                    options: data.options.clone(),
                    guild_id: self.guild_id.clone(),
                    channel_id: self.channel_id.clone(),
                };

                for command in commands.iter() {
                    if command.name() == data.name {
                        let data = command.respond(&input).await?;

                        return Ok( InteractionResponse {
                            interaction_callback_type: InteractionCallbackType::ChannelMessageWithSource,
                            data: Some(CallbackData::Message(data)),
                        })
                    }
                }
                Err(InteractionError::CommandNotFound(data.name.clone()))
            },
            _ => Err(InteractionError::Error("Interaction was not a command".to_string()))
        }
    }
    
    async fn handle_autocomplete(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, InteractionError> {
        let data = self.get_data().map_err(|_| InteractionError::Error("No Data".to_string()))?;
        match self.get_type() {
            InteractionType::ApplicationCommandAutocomplete => {
                let commands = get_commands();
                let input = Input {
                    ctx,
                    member: self.member.clone(),
                    user: self.user.clone(),
                    options: data.options.clone(),
                    guild_id: self.guild_id.clone(),
                    channel_id: self.channel_id.clone(),
                };

                for command in commands.iter() {
                    if command.name() == data.name {
                        let data = command.autocomplete(&input).await?;

                        return Ok( InteractionResponse {
                            interaction_callback_type: InteractionCallbackType::ApplicationCommandAutocompleteResult,
                            data: Some(CallbackData::Autocomplete(data)),
                        })
                    }
                }
                Err(InteractionError::CommandNotFound(data.name.clone()))
            },
            _ => Err(InteractionError::Error("".to_string()))
        }
    }

    pub async fn handle_interaction(&self, ctx: &mut worker::RouteContext<()>) -> Result<InteractionResponse, Error> {
        match self.interaction_type {
            InteractionType::Ping => Ok(self.ping_handler()),
            InteractionType::ApplicationCommand => self.command_handler(ctx).await.map_err(Error::InteractionFailed),
            InteractionType::ApplicationCommandAutocomplete => self.handle_autocomplete(ctx).await.map_err(Error::InteractionFailed),
            _ => Err(Error::PayloadError("Interaction not implemented".to_string())),
        }
    }
}
