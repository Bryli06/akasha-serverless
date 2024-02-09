use std::cmp;

use async_trait::async_trait;
use ndarray::{Array1, s, Array2, Axis, arr1};
use worker::console_log;
use crate::{discord::*, utils::InteractionError};
use super::{Command, Input, chances::round_sigfig, chance::five_star_character, chance::weapon};
use fftlib;

pub struct Pulls {}

#[async_trait(?Send)]
impl Command for Pulls {
    fn name(&self) -> String {
        "pulls".to_string()
    }

    fn description(&self) -> String {
        "Get the number of pulls needed for a character and weapon combinations with a specific probability".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![
            ApplicationCommandOption {
                name: "probability".to_string(),
                autocomplete: Some(false),
                description: "Probability you wish to need".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Number,
                choices: None,
                min_value: Some(0),
                max_value: Some(1),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "character_pity".to_string(),
                autocomplete: Some(false),
                description: "What character pity are you at right now?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,
                min_value: Some(0),
                max_value: Some(89),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "weapon_pity".to_string(),
                autocomplete: Some(false),
                description: "What character pity are you at right now?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,
                min_value: Some(0),
                max_value: Some(76),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "character_cons".to_string(),
                autocomplete: Some(false),
                description: "What character constellation are you aiming for?".to_string(),
                required: Some(false),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,
                min_value: Some(0),
                max_value: Some(6),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "weapon_refine".to_string(),
                autocomplete: Some(false),
                description: "What weapon refinement are you aiming for?".to_string(),
                required: Some(false),
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
        let cons = match input.get_options("character_cons") {
            Some(x) => x.as_u64().unwrap() as usize,
            None => 100,
        };
        let refine = match input.get_options("weapon_refine") {
            Some(x) => x.as_u64().unwrap() as usize,
            None => 100,
        };
        let prob = input.get_options("probability").unwrap().as_f64().unwrap();

        let embed = Embed {
            title: Some("Pulls necessary calculator".to_string()),
            embed_type: Some(EmbedType::Rich),
            description: Some("Want the inverse of this function? use /chance or /chances.".to_string()),
            url: None,
            color: Some(0x198754),
            footer: None,
            image: None,
            thumbnail: None,
            fields: Some(vec![EmbedField {
                name: match (cons, refine) {
                    (100, 100) => "Pulls necessary to get nothing".to_string(),
                    (100, x) => format!("Pulls necessary to get R{} with probability {}", x, prob),
                    (x, 100) => format!("Pulls necessary to get C{} with probability {}", x, prob),
                    (x,  y) => format!("Pulls necessary to get C{}R{} with probability {}", x, y, prob),
                },
                value: calc(prob,
                             input.get_options("character_pity").unwrap().as_u64().unwrap() as usize,
                             input.get_options("weapon_pity").unwrap().as_u64().unwrap() as usize,
                             refine, cons).to_string(),
                inline: Some(true),
            }]),
        };

        /*
            match input.get_options("banner").unwrap().as_u64() {
            Some(0) => five_star_character(
                input.get_options("wishes").unwrap().as_u64().unwrap() as usize,
                input.get_options("pity").unwrap().as_u64().unwrap(),
                input.get_options("guarantee").unwrap().as_u64().unwrap() as usize,),
            Some(1) => five_star_weapon(
                input.get_options("wishes").unwrap().as_u64().unwrap() as usize,
                input.get_options("pity").unwrap().as_u64().unwrap(),),
            _ => {
                console_log!("Unknown banner");
                (Some(1 << 6), Embed { //ephemeral
                    title: Some("Error".to_string()),
                    embed_type: Some(EmbedType::Rich),
                    description: Some("Got an unknown banner".to_string()),
                    url: None,
                    color: Some(0xcc0000),
                    footer: None,
                    image: None,
                    thumbnail: None,
                    fields: None,
                })
            }
        }; */

        Ok(MessagesInteractionCallbackData {
            content: None,
            components: None,
            embeds: Some(vec![embed]),
            attachments: None,
            flags: None,
        })
    }

}

fn calc(prob: f64, char_pity: usize, weapon_pity: usize, refine: usize, cons: usize) -> usize {
    assert!(prob >= 0.0 && prob <= 1.0); // 2508 * 2 + 78 + 91 - 1 = 5184 = ugly number
    assert!(char_pity < 90);
    assert!(weapon_pity < 77);
    let wishes = 2508;

    let arr = match (cons, refine) {
        (100, 100) => [1.0].to_vec(),
        (100, x) => weapon(weapon_pity,wishes, refine).slice(s![(weapon_pity) as i32..]).to_vec(),
        (x, 100) => five_star_character(char_pity, wishes, cons, false).slice(s![(char_pity) as i32..]).to_vec(),
        (x, y) => full(prob, wishes, char_pity, weapon_pity, refine, cons),
    };

    let mut sum = 0.0;
    let mut num = 0;
    while sum < prob && num <= wishes {
        sum += arr[num];
        num += 1;
    }
    num-1
}

fn full(prob: f64, wishes: usize, char_pity: usize, weapon_pity: usize, refine: usize, cons: usize) -> Vec<f64>{
    let temp1 = weapon(weapon_pity,wishes, refine);
    let weapon = temp1.slice(s![(weapon_pity) as i32..]);
    let temp2 = five_star_character(char_pity, wishes, cons, false);
    let char = temp2.slice(s![(char_pity) as i32..]);
    let a = weapon.as_slice().unwrap();
    let b = char.as_slice().unwrap();
    fftlib::fft::pmul(&a, &b)[..=wishes].to_vec()
}

