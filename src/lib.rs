use std::{collections::HashMap, error::Error, fs::File, io::Read, path::PathBuf, str::from_utf8};

use rand::seq::SliceRandom;
use regex::Regex;
use rust_embed::RustEmbed;

use crate::bubbles::{BubbleType, SpeechBubble};

pub mod bubbles;
pub mod errors;

#[derive(RustEmbed, Debug)]
#[folder = "src/charas"]
struct Asset;

/// Source chara to load, either builtin or from external file.
#[derive(Debug)]
pub enum Chara {
    All,
    Builtin(String),
    File(PathBuf),
    Raw(String),
    Random,
}

/// All built-in characters name.
pub const BUILTIN_CHARA: [&str; 23] = [
    "aya",
    "cirno",
    "clefairy",
    "cow",
    "eevee",
    "ferris",
    "ferris1",
    "flareon",
    "goldeen",
    "growlithe",
    "kirby",
    "kitten",
    "mario",
    "mew",
    "nemo",
    "pikachu",
    "piplup",
    "psyduck",
    "remilia-scarlet",
    "seaking",
    "togepi",
    "tux",
    "wartortle",
];

fn load_raw_chara_string(chara: &Chara) -> String {
    let mut raw_chara = String::new();

    match chara {
        Chara::File(s) => {
            let mut file = File::open(s).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
            file.read_to_string(&mut raw_chara)
                .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
        }

        Chara::Builtin(s) => {
            let name = format!("{}.chara", s);
            let asset = Asset::get(&name).unwrap();
            raw_chara = from_utf8(&asset.data)
                .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err))
                .to_string();
        }

        Chara::Raw(s) => {
            raw_chara = s.to_string();
        }

        Chara::All => {
            let charas = Asset::iter()
                .map(|file| {
                    let name = file.trim_end_matches(".chara");
                    let asset = Asset::get(&file).unwrap();
                    format!("{} 👇\n{}", name, String::from_utf8_lossy(&asset.data))
                })
                .collect::<Vec<_>>();
            raw_chara = charas.join("\n+\n");
        }

        Chara::Random => {
            let charas = Asset::iter().collect::<Vec<_>>();
            let choosen_chara = charas.choose(&mut rand::thread_rng()).unwrap().clone();
            let asset = Asset::get(&choosen_chara).unwrap();
            raw_chara = from_utf8(&asset.data)
                .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err))
                .to_string();
        }
    }

    raw_chara
}

fn strip_chara_string(raw_chara: &str) -> String {
    raw_chara
        .split('\n')
        .filter(|line| {
            !line.starts_with('#')
                && !line.starts_with("$x")
                && !line.contains("$thoughts")
                && !line.is_empty()
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\\e", "\x1B")
}

fn parse_character(chara: &Chara, voice_line: &str) -> String {
    let raw_chara = load_raw_chara_string(chara);
    let stripped_chara = strip_chara_string(&raw_chara);
    let charas = stripped_chara.split('+').collect::<Vec<_>>();
    let mut parsed = String::new();

    for chara in charas {
        // extract variable definition to HashMap
        let re = Regex::new(r"(?<var>\$\w).*=.*(?<val>\x1B\[.*m\s*).;").unwrap();
        let replacers: Vec<HashMap<&str, &str>> = re
            .captures_iter(chara)
            .map(|cap| {
                re.capture_names()
                    .flatten()
                    .filter_map(|n| Some((n, cap.name(n)?.as_str())))
                    .collect()
            })
            .collect();

        let mut chara_body = chara
            .split('\n')
            .filter(|line| !line.contains('=') && !line.contains("EOC"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim_end()
            .replace("$x", "\x1B[49m  ")
            .replace("$t", voice_line);

        // replace variable from character's body with actual value
        for replacer in replacers {
            chara_body = chara_body.replace(
                replacer.get("var").copied().unwrap(),
                replacer.get("val").copied().unwrap(),
            );
        }

        parsed.push_str(&format!("{}\n\n\n", &chara_body))
    }

    parsed.trim_end().to_string()
}

/// Format arguments to form complete charasay
pub fn format_character(
    messages: &str,
    chara: &Chara,
    max_width: usize,
    bubble_type: BubbleType,
) -> Result<String, Box<dyn Error>> {
    let voice_line: &str;
    let bubble_type = match bubble_type {
        BubbleType::Think => {
            voice_line = "o ";
            BubbleType::Think
        }
        BubbleType::Round => {
            voice_line = "╲ ";
            BubbleType::Round
        }
        BubbleType::Cowsay => {
            voice_line = "\\ ";
            BubbleType::Cowsay
        }
        BubbleType::Ascii => {
            voice_line = "\\ ";
            BubbleType::Ascii
        }
        BubbleType::Unicode => {
            voice_line = "╲ ";
            BubbleType::Unicode
        }
    };

    let speech_bubble = SpeechBubble::new(bubble_type);
    let speech = speech_bubble.create(messages, &max_width)?;
    let character = parse_character(chara, voice_line);

    Ok(format!("{}{}", speech, character))
}

/// Print only the character
pub fn print_character(chara: &Chara) -> String {
    parse_character(chara, "  ")
}
