use std::{collections::HashMap, fs::File, io::Read, path::PathBuf, str::from_utf8};

use rand::seq::SliceRandom;
use regex::Regex;
use rust_embed::RustEmbed;
use strip_ansi_escapes::strip;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

#[derive(RustEmbed, Debug)]
#[folder = "src/charas"]
struct Asset;

#[derive(Debug)]
enum BubbleType {
    Think,
    Round,
}

/// Source chara to load, either builtin or from external file.
#[derive(Debug)]
pub enum Chara {
    All,
    Builtin(String),
    File(PathBuf),
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

#[derive(Debug)]
struct SpeechBubble {
    corner_top_left: &'static str,
    top: &'static str,
    corner_top_right: &'static str,
    top_right: &'static str,
    right: &'static str,
    bottom_right: &'static str,
    corner_bottom_right: &'static str,
    bottom: &'static str,
    corner_bottom_left: &'static str,
    bottom_left: &'static str,
    left: &'static str,
    top_left: &'static str,
    short_left: &'static str,
    short_right: &'static str,
}

impl SpeechBubble {
    fn new(bubble_type: BubbleType) -> Self {
        let corner_top_left;
        let top;
        let corner_top_right;
        let top_right;
        let right;
        let bottom_right;
        let corner_bottom_right;
        let bottom;
        let corner_bottom_left;
        let bottom_left;
        let left;
        let top_left;
        let short_left;
        let short_right;

        match bubble_type {
            BubbleType::Think => {
                corner_top_left = "(";
                top = "⁀";
                corner_top_right = ")\n";
                top_right = "  )\n";
                right = "  )\n";
                bottom_right = "  )\n";
                corner_bottom_right = ")\n";
                bottom = "‿";
                corner_bottom_left = "(";
                bottom_left = "(  ";
                left = "(  ";
                top_left = "(  ";
                short_left = "(  ";
                short_right = "  )\n";
            }
            BubbleType::Round => {
                corner_top_left = "╭";
                top = "─";
                corner_top_right = "╮\n";
                top_right = "  │\n";
                right = "  │\n";
                bottom_right = "  │\n";
                corner_bottom_right = "╯\n";
                bottom = "─";
                corner_bottom_left = "╰";
                bottom_left = "│  ";
                left = "│  ";
                top_left = "│  ";
                short_left = "│  ";
                short_right = "  │\n";
            }
        };

        Self {
            corner_top_left,
            top,
            corner_top_right,
            top_right,
            right,
            bottom_right,
            corner_bottom_right,
            bottom,
            corner_bottom_left,
            bottom_left,
            left,
            top_left,
            short_left,
            short_right,
        }
    }

    fn line_len(line: &str) -> usize {
        let stripped = strip(line).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
        let text =
            from_utf8(stripped.as_slice()).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

        UnicodeWidthStr::width(text)
    }

    fn longest_line(lines: &[&str]) -> usize {
        lines
            .iter()
            .map(|line| Self::line_len(line))
            .max()
            .unwrap_or(0)
    }

    fn create(self, messages: &str, max_width: &usize) -> String {
        const SPACE: &str = " ";
        let mut write_buffer = Vec::new();

        // for computing messages length
        let wrapped = fill(messages, *max_width).replace('\t', "    ");
        let lines: Vec<&str> = wrapped.lines().collect();
        let line_count = lines.len();
        let actual_width = Self::longest_line(&lines);

        // draw top box border
        write_buffer.push(self.corner_top_left);
        for _ in 0..(actual_width + 4) {
            write_buffer.push(self.top);
        }
        write_buffer.push(self.corner_top_right);

        // draw inner message each line
        for (i, line) in lines.into_iter().enumerate() {
            // left border
            if line_count == 1 {
                write_buffer.push(self.short_left);
            } else if i == 0 {
                write_buffer.push(self.top_left);
            } else if i == line_count - 1 {
                write_buffer.push(self.bottom_left);
            } else {
                write_buffer.push(self.left);
            }

            // text line
            let line_len = Self::line_len(line);
            write_buffer.push(line);
            write_buffer.resize(write_buffer.len() + actual_width - line_len, SPACE);

            // right border
            if line_count == 1 {
                write_buffer.push(self.short_right);
            } else if i == 0 {
                write_buffer.push(self.top_right);
            } else if i == line_count - 1 {
                write_buffer.push(self.bottom_right);
            } else {
                write_buffer.push(self.right);
            }
        }

        // draw bottom box border
        write_buffer.push(self.corner_bottom_left);
        for _ in 0..(actual_width + 4) {
            write_buffer.push(self.bottom);
        }
        write_buffer.push(self.corner_bottom_right);

        write_buffer.join("")
    }
}

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

        Chara::All => todo!(),

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

    // extract variable definition to HashMap
    let re = Regex::new(r"(?P<var>\$\w).*=.*(?P<val>\x1B\[.*m\s*).;").unwrap();
    let replacers: Vec<HashMap<&str, &str>> = re
        .captures_iter(&stripped_chara)
        .map(|cap| {
            re.capture_names()
                .flatten()
                .filter_map(|n| Some((n, cap.name(n)?.as_str())))
                .collect()
        })
        .collect();

    let mut chara_body = stripped_chara
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

    chara_body
}

/// Format arguments to form complete charasay
pub fn format_character(messages: &str, chara: &Chara, max_width: usize, think: bool) -> String {
    let voice_line = if think { "o " } else { "╲ " };
    let bubble_type = if think {
        BubbleType::Think
    } else {
        BubbleType::Round
    };

    let speech_bubble = SpeechBubble::new(bubble_type);

    let speech = speech_bubble.create(messages, &max_width);
    let character = parse_character(chara, voice_line);

    format!("{}{}", speech, character)
}

/// Print only the character
pub fn print_character(chara: &Chara) -> String {
    parse_character(chara, " ")
}
