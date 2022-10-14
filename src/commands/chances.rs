use async_trait::async_trait;
use worker::console_log;
use crate::{discord::*, utils::InteractionError};
use super::{Command, Input};

pub struct Chances {}

#[async_trait(?Send)]
impl Command for Chances {
    fn name(&self) -> String {
        "chances".to_string()
    }

    fn description(&self) -> String {
        "Get the probalitity to get desired 5 star".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![
            ApplicationCommandOption {
                name: "banner".to_string(),
                autocomplete: Some(false),
                description: "Which banner are you rolling on?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::String,
                choices: Some(vec![
                    ApplicationCommandOptionChoice {
                        name: "5 star Character".to_string(),
                        value: "0".to_string(),
                    },
                    ApplicationCommandOptionChoice {
                        name: "5 star Weapon".to_string(),
                        value: "1".to_string(), 
                    },
                ]),  
                min_value: None,
                max_value: None,
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "wishes".to_string(),
                autocomplete: Some(false),
                description: "How many wishes do you have?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,  
                min_value: Some(0),
                max_value: Some(1260),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "pity".to_string(),
                autocomplete: Some(false),
                description: "What pity are you at right now?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,  
                min_value: Some(0),
                max_value: Some(90),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "guarentee".to_string(),
                autocomplete: Some(false),
                description: "Do you have guarentee or are you at 50/50".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::String,
                choices: Some(vec![
                    ApplicationCommandOptionChoice {
                        name: "Yes".to_string(),
                        value: "1".to_string(),
                    },
                    ApplicationCommandOptionChoice {
                        name: "No".to_string(),
                        value: "0".to_string(), 
                    },
                    ApplicationCommandOptionChoice {
                        name: "N/A".to_string(),
                        value: "0".to_string(),
                    },
                ]),  
                min_value: None,
                max_value: None,
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
        let embed = match input.get_options("banner") {
            Some("0") => five_star_character(
                parse_to_u32(input.get_options("wishes")), 
                parse_to_u32(input.get_options("pity")), 
                parse_to_u32(input.get_options("guarentee")),),
            Some("1") => five_star_weapon(
                parse_to_u32(input.get_options("wishes")), 
                parse_to_u32(input.get_options("pity"))),
            _ => {
                console_log!("Unknown banner");
                Embed {
                    title: Some("Error".to_string()),
                    embed_type: Some(EmbedType::Rich),
                    description: Some("Got an unknown banner".to_string()),
                    url: None,
                    color: Some(0xcc0000),
                    footer: None,
                    image: None,
                    thumbnail: None, 
                    fields: None,
                }
            }
        };

        Ok(MessagesInteractionCallbackData {
            content: None,
            components: None,
            embeds: Some(vec![embed]),
            attachment: None,
        })
    }

}

fn parse_to_u32(s: Option<&str>) -> u32 {
    match s {
        Some(s) => s.parse().unwrap(),
        None => 0,
    }
}

fn five_star_character(wishes: u32, pity: u32, guarentee: u32) -> Embed {
    Embed {
        title: Some("Hello".to_string()),
        embed_type: Some(EmbedType::Rich),
        description: Some("E".to_string()),
        url: None,
        color: Some(0x696969),
        footer: None,
        image: None,
        thumbnail: None, 
        fields: Some(vec![
            EmbedField {
                name: "Wishes".to_string(),
                value: format!("{}", wishes),
                inline: Some(true),
            }, 
            EmbedField {
                name: "Pity".to_string(),
                value: format!("{}", pity),
                inline: Some(true),
            },
            EmbedField {
                name: "Guarentee".to_string(),
                value: format!("{}", guarentee),
                inline: Some(true),
            }
        ]),
    }
}

fn five_star_weapon(wishes: u32, pity: u32) -> Embed {
    Embed {
        title: Some("Hello".to_string()),
        embed_type: Some(EmbedType::Rich),
        description: Some("E".to_string()),
        url: None,
        color: Some(0x696969),
        footer: None,
        image: None,
        thumbnail: None, 
        fields: Some(vec![
            EmbedField {
                name: "Wishes".to_string(),
                value: format!("{}", wishes),
                inline: Some(true),
            }, 
            EmbedField {
                name: "Pity".to_string(),
                value: format!("{}", pity),
                inline: Some(true),
            },
        ]),
    }
}
