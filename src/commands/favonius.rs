use std::cmp;

use async_trait::async_trait;
use worker::console_log;
use crate::{discord::*, utils::InteractionError};
use super::{Command, Input};

pub struct Favonius {}

#[async_trait(?Send)]
impl Command for Favonius {
    fn name(&self) -> String {
        "favonius".to_string()
    }

    fn description(&self) -> String {
        "Get the probalitity that favonius sword procs".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![
            ApplicationCommandOption {
                name: "crit".to_string(),
                autocomplete: Some(false),
                description: "What is your character's crit rate?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Number,
                choices: None,  
                min_value: Some(0),
                max_value: Some(100),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "hits".to_string(),
                autocomplete: Some(false),
                description: "How many hits per proc?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,  
                min_value: Some(1),
                max_value: None,
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "refinement".to_string(),
                autocomplete: Some(false),
                description: "What refinement is the weapon at?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,  
                min_value: Some(1),
                max_value: Some(5),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
        ]) 
    }

    async fn autocomplete(&self, input: &super::Input) -> Result<Option<crate::discord::AutocompleteInteractionCallbackData>, crate::utils::InteractionError> {
        Ok(None)
    }

    async fn respond(&self, input: &Input) -> Result<MessagesInteractionCallbackData, InteractionError> {
        let embed = proc_chance(
                input.get_options("crit").unwrap().as_f64().unwrap(), 
                input.get_options("hits").unwrap().as_u64().unwrap() as i32, 
                input.get_options("refinement").unwrap().as_u64().unwrap(),
            );

        Ok(MessagesInteractionCallbackData {
            content: None,
            components: None,
            embeds: Some(vec![embed]),
            attachments: None,
            flags: None,
        })
    }

}

fn proc_chance(cr: f64, hits: i32, refinement: u64) -> Embed {
    Embed {
        title: Some("Favonius proc chances calculator".to_string()),
        embed_type: Some(EmbedType::Rich),
        description: Some(format!("At R{} with a crit rate of {}%, there is a {:.2}% chance to trigger in {} hits.",
                                  refinement, cr, 100.0*(1.0-(1.0-(cr / 100.0 * (50.0 + 10.0 * refinement as f64)/100.0)).powi(hits)), hits)),
        url: None,
        color: Some(0x198754),
        footer: None,
        image: None,
        thumbnail: None, 
        fields: None,
    }
}

fn round_sigfig(num: f64, sigfigs: isize) -> String {
    if num == 0.0 {
        return "0.0000%".to_string()
    }
    let leading_digits = num.abs().log10().ceil() as isize; // Treat fractions as negative leading digits so we have the correct total number of significant digits
    let trailing_digits = if leading_digits > sigfigs { 0 } else { (sigfigs - leading_digits) as usize };
    
    format!("{:.*}%", trailing_digits, num)
}
