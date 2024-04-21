use std::cmp;

use async_trait::async_trait;
use ndarray::{Array1, s, Array2, Axis, arr1};
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
                min_value: Some(1),
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
                max_value: Some(89),
                min_length: None,
                max_length: None,
                options: None,
                channel_types: None,
            },
            ApplicationCommandOption {
                name: "guarantee".to_string(),
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
        let (flags, embed) = match input.get_options("banner").unwrap().as_u64() {
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
        };

        Ok(MessagesInteractionCallbackData {
            content: None,
            components: None,
            embeds: Some(vec![embed]),
            attachments: None,
            flags,
        })
    }

}


fn five_star_character(wishes: usize, pity: u64, guarentee: usize) -> (Option<u64>, Embed) {
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
        for j in 1..cmp::min(90*i+1, wishes+pity as usize) {
            let temp = gf_coefficents[[i-1, j]];
            gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 91) as i32]).scaled_add(temp, &base_gf_coefficents);
        }
    }

    let five_star_prob = gf_coefficents.slice(s![.., ..(wishes+pity as usize+1)]).sum_axis(Axis(1));

    let mut cons: Vec<EmbedField> = Vec::new();
    for i in 0..7 {
        let mut temp = 0.0;
        for j in 0..(i+2-guarentee) {
            temp += 100.0 * (1..=j.min((i + 1 - guarentee) - j))
                .fold(1, |acc, val| acc * ((i + 1 - guarentee) - val + 1) / val) as f64
                / (1 << (i + 1 - guarentee)) as f64 * five_star_prob[i+j];
        }
        cons.push(EmbedField {
            name: format!("C{}", i),
            value: round_sigfig(temp, 4),
            inline: Some(true),
        })
    }

    (None, Embed {
        title: Some("Character chances calculator".to_string()),
        embed_type: Some(EmbedType::Rich),
        description: Some("If you wish to understand the math behind this calculation, view the explanation [here](https://drive.google.com/file/d/1EECcjNVpfiOTqRoS48hHWqH2Ake902vq/view?usp=sharing). Looking to pull for both a character and weapon? Use /chance (no s).".to_string()),
        url: None,
        color: Some(0x198754),
        footer: None,
        image: None,
        thumbnail: None,
        fields: Some(cons),
    })
}

fn five_star_weapon(wishes: usize, pity: u64) -> (Option<u64>, Embed) {
    if pity >= 77 {
        return  (Some(1 << 6), Embed { //ephemeral
                    title: Some("Error".to_string()),
                    embed_type: Some(EmbedType::Rich),
                    description: Some(format!("Pity {} is greater than the maximum weapon pity of 77.", pity)),
                    url: None,
                    color: Some(0xcc0000),
                    footer: None,
                    image: None,
                    thumbnail: None,
                    fields: None,
                })
    }

    let P = 0.007;
    let ramp_rate = 0.07;

    let base_gf_coefficents = { //scope so it gets dropped
        let mut cum_prob = Array1::<f64>::zeros(78);

        cum_prob[0] = 0.0;
        cum_prob.slice_mut(s![1..63 as i32]).fill(P);
        cum_prob[77] = 1.0;

        for i in 63..77 {
            cum_prob[i] = P + ramp_rate * (i-62) as f64;
        }

        let mut temp = Array1::<f64>::zeros(78);
        let mut cum_product = 1.0;
        for i in 0..78 {
            temp[i] = cum_product * cum_prob[i];
            cum_product *= 1.0 - cum_prob[i];
        }
        temp
    };
    let pity_sum = base_gf_coefficents.slice(s![1.. (pity+1) as i32]).sum();

    let mut gf_coefficents = Array2::<f64>::zeros((15, pity as usize + wishes + 79));

    gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![(pity+1) as i32 .. 78]).assign(&(&base_gf_coefficents.slice(s![(pity+1) as i32 .. ])/(1.0 - pity_sum)));

    for i in 1..15 {
        for j in 1..cmp::min(77*i+1, wishes+pity as usize) {
            let temp = gf_coefficents[[i-1, j]];
            gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 78) as i32]).scaled_add(temp, &base_gf_coefficents);
        }
    }

    let five_star_prob = gf_coefficents.slice(s![.., ..(wishes+pity as usize+1)]).sum_axis(Axis(1));

    let mut path_gf_coefficents = Array2::<f64>::zeros((5, 16));

    path_gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![0i32..4]).assign(&arr1(&[0.0, 0.375, 0.265625, 0.359375]));

    for i in 1..5 {
        for j in 1..3*i+1 {
            let temp = path_gf_coefficents[[i-1, j]];

            path_gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 4) as i32]).scaled_add(temp, &arr1(&[0.0, 0.375, 0.265625, 0.359375]));
        }
    }

    let mut cons: Vec<EmbedField> = Vec::new();
    for i in 0..5 {
        cons.push(EmbedField {
            name: format!("R{}", i+1),
            value: round_sigfig(100.0 * path_gf_coefficents.index_axis_mut(Axis(0), i).slice(s![1i32..])
                .dot(&five_star_prob), 4),
            inline: Some(true),
        })
    }

    (None, Embed {
        title: Some("Weapon chances calculator".to_string()),
        embed_type: Some(EmbedType::Rich),
        description: Some("If you wish to understand the math behind this calculation, view the explanation [here](https://drive.google.com/file/d/1EECcjNVpfiOTqRoS48hHWqH2Ake902vq/view?usp=sharing). Looking to pull for both a character and weapon? Use /chance (no s).".to_string()),
        url: None,
        color: Some(0x198754),
        footer: None,
        image: None,
        thumbnail: None,
        fields: Some(cons),
    })
}

pub fn round_sigfig(num: f64, sigfigs: isize) -> String {
    if num == 0.0 {
        return "0.0000%".to_string()
    }
    let leading_digits = num.abs().log10().ceil() as isize; // Treat fractions as negative leading digits so we have the correct total number of significant digits
    let trailing_digits = if leading_digits > sigfigs { 0 } else { (sigfigs - leading_digits) as usize };

    format!("{:.*}%", trailing_digits, num)
}
