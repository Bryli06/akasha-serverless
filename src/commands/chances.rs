use std::cmp;

use async_trait::async_trait;
use ndarray::{Array1, s, Array2, Axis, Slice};
use worker::console_log;
use crate::{discord::*, utils::InteractionError};
use serde_json::value::Value;
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
                option_type: ApplicationCommandOptionType::Integer,
                choices: Some(vec![
                    ApplicationCommandOptionChoice {
                        name: "5 star Character".to_string(),
                        value: 0.into(),
                    },
                    ApplicationCommandOptionChoice {
                        name: "5 star Weapon".to_string(),
                        value: 1.into(), 
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
                option_type: ApplicationCommandOptionType::Integer,
                choices: Some(vec![
                    ApplicationCommandOptionChoice {
                        name: "Yes".to_string(),
                        value: 1.into(),
                    },
                    ApplicationCommandOptionChoice {
                        name: "No".to_string(),
                        value: 0.into(), 
                    },
                    ApplicationCommandOptionChoice {
                        name: "N/A".to_string(),
                        value: 0.into(),
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
        let embed = match input.get_options("banner").unwrap().as_u64() {
            Some(0) => five_star_character(
                input.get_options("wishes").unwrap().as_u64().unwrap() as usize, 
                input.get_options("pity").unwrap().as_u64().unwrap(), 
                input.get_options("guarentee").unwrap().as_u64().unwrap(),),
            Some(1) => five_star_weapon(
                input.get_options("wishes").unwrap().as_u64().unwrap(), 
                input.get_options("pity").unwrap().as_u64().unwrap(),),
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


fn five_star_character(wishes: usize, pity: u64, guarentee: u64) -> Embed {
    let P = 0.006;
    let ramp_rate = 0.06;

    let base_gf_coefficents = { //scope so it gets dropped
        let mut cum_prob = Array1::<f64>::zeros(91);

        cum_prob[0] = 0.0;
        cum_prob.slice_mut(s![1..74 as i32]).fill(P);
        cum_prob[90] = 1.0;

        for i in 74..90 {
            cum_prob[i] = P + ramp_rate * (i-73) as f64;
        }

        let mut temp = Array1::<f64>::zeros(91);
        let mut cum_product = 1.0;
        for i in 0..91 {
            temp[i] = cum_product * cum_prob[i];
            cum_product *= 1.0 - cum_prob[i];
        }
        temp
    };
    let pity_sum = base_gf_coefficents.slice(s![1.. (pity+1) as i32]).sum();

    let mut gf_coefficents = Array2::<f64>::zeros((14, pity as usize + wishes + 92));

    gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![(pity+1) as i32 .. 91]).assign(&(&base_gf_coefficents.slice(s![(pity+1) as i32 .. 91])/(1.0 - pity_sum)));

    for i in 1..14 {
        println!("{:?}", gf_coefficents);
        for j in 1..cmp::min(90*i+1, wishes+pity as usize) {
            let temp = gf_coefficents[[i-1, j]];
            gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 91) as i32]).scaled_add(temp, &base_gf_coefficents);
        }
    }

    let five_star_prob = gf_coefficents.slice(s![.., ..(wishes+pity as usize+1)]).sum_axis(Axis(1));

    console_log!("{:?}", five_star_prob);
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

fn five_star_weapon(wishes: u64, pity: u64) -> Embed {
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
