use std::cmp;

use async_trait::async_trait;
use ndarray::{Array1, s, Array2, Axis, arr1};
use worker::console_log;
use crate::{discord::*, utils::InteractionError};
use super::{Command, Input, chances::round_sigfig};
use fftlib;

pub struct Chance {}

#[async_trait(?Send)]
impl Command for Chance {
    fn name(&self) -> String {
        "chance".to_string()
    }

    fn description(&self) -> String {
        "Get the specific probabilty to pull a character and weapon combonation".to_string()
    }

    fn options(&self) -> Option<Vec<ApplicationCommandOption>> {
        Some(vec![
            ApplicationCommandOption {
                name: "wishes".to_string(),
                autocomplete: Some(false),
                description: "How many wishes do you have?".to_string(),
                required: Some(true),
                option_type: ApplicationCommandOptionType::Integer,
                choices: None,
                min_value: Some(1),
                max_value: Some(2507),
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
                required: Some(true),
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
        let cons = input.get_options("character_cons").unwrap().as_u64().unwrap() as usize;
        let refine = input.get_options("weapon_refine").unwrap().as_u64().unwrap() as usize;
        let wishes = input.get_options("wishes").unwrap().as_u64().unwrap() as usize;

        let embed = Embed {
            title: Some("Combined chances calculator".to_string()),
            embed_type: Some(EmbedType::Rich),
            description: Some("How does this calculation work? FREAKING MAGIC IDK. Looking for a detailed look at either characters or weapons? Use /chance**s**.".to_string()),
            url: None,
            color: Some(0x198754),
            footer: None,
            image: None,
            thumbnail: None,
            fields: Some(vec![EmbedField {
                name: format!("C{}R{} with {} wishes", cons, refine, wishes),
                value: round_sigfig(100.0 * full(wishes,
                                                 input.get_options("character_pity").unwrap().as_u64().unwrap() as usize,
                                                 input.get_options("weapon_pity").unwrap().as_u64().unwrap() as usize,
                                                 refine, cons), 4),
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

fn full(wishes: usize, char_pity: usize, weapon_pity: usize, refine: usize, cons: usize) -> f64 {
    assert!(wishes < 2508); // 2508 * 2 + 78 + 91 - 1 = 5184 = ugly number
    assert!(char_pity < 90);
    assert!(weapon_pity < 77);
    let temp1 = weapon(weapon_pity,wishes, refine);
    let weapon = temp1.slice(s![(weapon_pity) as i32..]);
    let temp2 = five_star_character(char_pity, wishes, cons, false);
    let char = temp2.slice(s![(char_pity) as i32..]);
    let a = weapon.as_slice().unwrap();
    let b = char.as_slice().unwrap();
    fftlib::fft::pmul(&a, &b)[..=wishes].into_iter().sum::<f64>()
}

pub fn five_star_character(pity: usize, wishes: usize, cons: usize, guarentee: bool) -> Array1<f64> {
    let p = 0.006;
    let ramp_rate = 0.06;

    let base_gf_coefficents = { //scope so it gets dropped
        let mut cum_prob = Array1::<f64>::zeros(91);

        cum_prob[0] = 0.0;
        cum_prob.slice_mut(s![1..74 as i32]).fill(p);
        cum_prob[90] = 1.0;

        for i in 74..90 {
            cum_prob[i] = p + ramp_rate * (i-73) as f64;
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

    let mut gf_coefficents = Array2::<f64>::zeros((2*cons+2, pity as usize + wishes + 92));

    gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![(pity+1) as i32 .. 91]).assign(&(&base_gf_coefficents.slice(s![(pity+1) as i32 .. 91])/(1.0 - pity_sum)));

    for i in 1..2*cons + 2 {
        for j in 1..cmp::min(90*i+1, wishes+pity as usize) {
            let temp = gf_coefficents[[i-1, j]];
            gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 91) as i32]).scaled_add(temp, &base_gf_coefficents);
        }
    }

    let mut path_gf_coefficents = Array2::<f64>::zeros((cons + 1, 2*cons+3));

    path_gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![0i32..3]).assign(&arr1(
            if guarentee {
                &[0.0, 1.0, 0.0]
            }
            else {
                &[0.0, 0.5, 0.5]
            }));

    for i in 1..cons + 1{
        for j in 1..2*i+1 {
            let temp = path_gf_coefficents[[i-1, j]];

            path_gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 3) as i32]).scaled_add(temp, &arr1(&[0.0, 0.5, 0.5]));
        }
    }

    let mut ans = Array1::<f64>::zeros(pity + wishes + 92);

    path_gf_coefficents.index_axis(Axis(0), cons).slice(s![1i32..]).iter().enumerate().for_each(|(i, x)| {
        ans.scaled_add(*x, &gf_coefficents.index_axis_mut(Axis(0), i));
    });

    ans
}

pub fn weapon(pity: usize, wishes: usize, refine: usize) -> Array1<f64> {

    let p = 0.007;
    let ramp_rate = 0.07;

    let base_gf_coefficents = { //scope so it gets dropped
        let mut cum_prob = Array1::<f64>::zeros(78);

        cum_prob[0] = 0.0;
        cum_prob.slice_mut(s![1..63 as i32]).fill(p);
        cum_prob[77] = 1.0;

        for i in 63..77 {
            cum_prob[i] = p + ramp_rate * (i-62) as f64;
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

    let mut gf_coefficents = Array2::<f64>::zeros((3*refine, pity as usize + wishes + 79));

    gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![(pity+1) as i32 .. 78]).assign(&(&base_gf_coefficents.slice(s![(pity+1) as i32 .. ])/(1.0 - pity_sum)));

    for i in 1..3*refine {
        for j in 1..cmp::min(77*i+1, wishes+pity as usize) {
            let temp = gf_coefficents[[i-1, j]];
            gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 78) as i32]).scaled_add(temp, &base_gf_coefficents);
        }
    }

    let mut path_gf_coefficents = Array2::<f64>::zeros((5, 3*refine + 1));

    path_gf_coefficents.index_axis_mut(Axis(0), 0).slice_mut(s![0i32..4]).assign(&arr1(&[0.0, 0.375, 0.265625, 0.359375]));

    for i in 1..refine {
        for j in 1..3*i+1 {
            let temp = path_gf_coefficents[[i-1, j]];

            path_gf_coefficents.index_axis_mut(Axis(0), i).slice_mut(s![j as i32 .. (j + 4) as i32]).scaled_add(temp, &arr1(&[0.0, 0.375, 0.265625, 0.359375]));
        }
    }

    let mut ans = Array1::<f64>::zeros(pity + wishes + 79);

    path_gf_coefficents.index_axis(Axis(0), refine-1).slice(s![1i32..]).iter().enumerate().for_each(|(i, x)| {
        ans.scaled_add(*x, &gf_coefficents.index_axis_mut(Axis(0), i));
    });

    ans
}
