use async_trait::async_trait;
use serde_json::value::Value;
use crate::discord::*;

use crate::utils::InteractionError;

mod chances;    
mod invite;
mod about;
mod characters;


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

    async fn respond(&self, input: &Input) -> Result<MessagesInteractionCallbackData, InteractionError> {
        unimplemented!()
    }
}

pub struct Input<'t> {
    pub ctx: &'t mut worker::RouteContext<()>,
    pub member: Option<Member>,
    pub user: Option<User>,
    pub options: Option<Vec<ApplicationCommandInteractionDataOption>>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
}

impl Input<'_> {
    pub async fn kv_get(&self, namespace: &str, key: &str) -> Result<Option<String>, InteractionError> {
        let kv = self.ctx.kv(namespace).map_err( |_|InteractionError::CloudflareError("Connecting to KV".into()))?;
        let value = kv.get(key).text().await.map_err( |_|InteractionError::CloudflareError("Fetching from KV".into()))?;
        Ok(value)
    }

    pub async fn kv_put(&self, namespace: &str, key: &str, value: &str) -> Result<(), InteractionError> {
        let kv = self.ctx.kv(namespace).map_err( |_|InteractionError::CloudflareError("Connecting to KV".into()))?;
        kv.put(key, value)
        .map_err( |_|InteractionError::CloudflareError("bind to KV".into()))?
        .execute()
        .await
        .map_err(|_| InteractionError::CloudflareError("KV put".into()))
        ?;
        Ok(())
    }

    pub fn get_options(&self, name: &str) -> Option<&Value> {
        match &self.options {
            Some(options) => {
                for option in options {
                    if option.name == name {
                        match option.value {
                            Some(ref value) => return Some(value),
                            None => return None,
                        }
                    }
                }
                None
            },
            None => None
        }
    }
}

pub fn get_commands() -> Vec<Box<dyn Command + Sync>> {
    vec![
        Box::new(chances::Chances {}),
        Box::new(invite::Invite {}),
        Box::new(about::About {}),
    ]
}
