use ed25519_dalek::{PublicKey, Verifier, Signature};
use worker::{Request, RouteContext};

use crate::{discord::{InteractionResponse, Interaction}, utils::{HttpError, Error, VerificationError}};

pub struct Bot {
    request: Request,
    ctx: RouteContext<()>,
}

impl Bot {
    pub fn new(request: Request, ctx: RouteContext<()>) -> Bot {
        Bot{request, ctx}
    }
    
    fn get_enviromental_variable(&self, key: &str) -> Result<String, Error> {
        match self.ctx.var(key) {
            Ok(var) => Ok(var.to_string()),
            Err(_) => Err(Error::EnvironmentVariableNotFound(key.to_string())),
        }
    }

    fn get_header(&self, key: &str) -> Result<String, Error> { // i hate astraction 
        match self.request.headers().get(key) {
            Ok(header) => header.ok_or_else(|| Error::HeaderNotFound(key.to_string())),
            Err(_) => Err(Error::HeaderNotFound(key.to_string())),
        }
    }
    pub async fn handle_interaction(&mut self) -> Result<InteractionResponse, HttpError> {
        let key = &(hex::decode(self.
            get_enviromental_variable("DISCORD_KEY")?)
            .map_err(VerificationError::ParseError)
            .and_then( |bytes| {
                PublicKey::from_bytes(&bytes).map_err(VerificationError::InvalidSignature)
            })
            .map_err(Error::VerificationFailed)?);

        let signature = self.get_header("x-signature-ed25519")?;
        let timestamp = self.get_header("x-signature-timestamp")?;
        let body = self.request.text().await.map_err(|_| Error::PayloadError("".to_string()))?;

        key.verify(format!("{}{}", timestamp, &body).as_bytes(), 
            &hex::decode(&signature)
                .map_err(VerificationError::ParseError)
                .and_then( |bytes| {
                    Signature::from_bytes(&bytes)
                        .map_err(VerificationError::InvalidSignature)
                }).map_err(Error::VerificationFailed)?,)
                .map_err(VerificationError::InvalidSignature)
                .map_err(Error::VerificationFailed)?; //what the actual fuck this is what no try catch does to a mf

        
        worker::console_log!("Body: {}", body);

        let interaction = serde_json::from_str::<Interaction>(&body)
            .map_err(Error::JsonFailed)?;


        worker::console_log!("Parse: {}", serde_json::to_string_pretty(&interaction).unwrap());

        Ok(interaction.handle_interaction(&mut self.ctx).await?)

    }
}
