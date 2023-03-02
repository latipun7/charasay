use std::{collections::HashMap, fs::File, io::Read, str::from_utf8};

use regex::Regex;
use rust_embed::RustEmbed;
use strip_ansi_escapes::strip;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

#[derive(RustEmbed, Debug)]
#[folder = "$CARGO_MANIFEST_DIR/src/charas"]
struct Asset;

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

const SPACE: &str = " ";
const ROUND_BUBBLE: SpeechBubble = SpeechBubble {
    corner_top_left: "╭",
    top: "─",
    corner_top_right: "╮\n",
    top_right: "  │\n",
    right: "  │\n",
    bottom_right: "  │\n",
    corner_bottom_right: "╯\n",
    bottom: "─",
    corner_bottom_left: "╰",
    bottom_left: "│  ",
    left: "│  ",
    top_left: "│  ",
    short_left: "│  ",
    short_right: "  │\n",
};
const THINK_BUBBLE: SpeechBubble = SpeechBubble {
    corner_top_left: "(",
    top: "⁀",
    corner_top_right: ")\n",
    top_right: "  )\n",
    right: "  )\n",
    bottom_right: "  )\n",
    corner_bottom_right: ")\n",
    bottom: "‿",
    corner_bottom_left: "(",
    bottom_left: "(  ",
    left: "(  ",
    top_left: "(  ",
    short_left: "(  ",
    short_right: "  )\n",
};

fn line_len(line: &str) -> usize {
    let stripped = strip(line).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
    let text = from_utf8(stripped.as_slice()).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

    UnicodeWidthStr::width(text)
}

fn longest_line(lines: &[&str]) -> usize {
    lines.iter().map(|line| line_len(line)).max().unwrap_or(0)
}

fn create_speech_bubble(messages: &str, max_width: usize, think: bool) -> String {
    let mut write_buffer = Vec::new();

    let speech_bubble: SpeechBubble = if think { THINK_BUBBLE } else { ROUND_BUBBLE };

    // Let textwrap work its magic
    let wrapped = fill(messages, max_width).replace('\t', "    ");

    let lines: Vec<&str> = wrapped.lines().collect();

    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    // top box border
    write_buffer.push(speech_bubble.corner_top_left);
    for _ in 0..(actual_width + 4) {
        write_buffer.push(speech_bubble.top);
    }
    write_buffer.push(speech_bubble.corner_top_right);

    // inner message
    for (i, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.push(speech_bubble.short_left);
        } else if i == 0 {
            write_buffer.push(speech_bubble.top_left);
        } else if i == line_count - 1 {
            write_buffer.push(speech_bubble.bottom_left);
        } else {
            write_buffer.push(speech_bubble.left);
        }

        let line_len = line_len(line);
        write_buffer.push(line);
        write_buffer.resize(write_buffer.len() + actual_width - line_len, SPACE);

        if line_count == 1 {
            write_buffer.push(speech_bubble.short_right);
        } else if i == 0 {
            write_buffer.push(speech_bubble.top_right);
        } else if i == line_count - 1 {
            write_buffer.push(speech_bubble.bottom_right);
        } else {
            write_buffer.push(speech_bubble.right);
        }
    }

    // bottom box border
    write_buffer.push(speech_bubble.corner_bottom_left);
    for _ in 0..(actual_width + 4) {
        write_buffer.push(speech_bubble.bottom);
    }
    write_buffer.push(speech_bubble.corner_bottom_right);

    write_buffer.join("")
}

fn parse_character(chara: &String, voice_line: &str) -> String {
    // Get raw text
    let mut raw_chara = String::new();
    match chara.contains(".chara") {
        true => {
            let mut file = File::open(chara).unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
            file.read_to_string(&mut raw_chara)
                .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));
        }
        false => {
            let name = format!("{}.chara", &chara);
            let asset = Asset::get(&name).unwrap();
            raw_chara = from_utf8(&asset.data)
                .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err))
                .to_string();
        }
    }

    let stripped_chara = raw_chara
        .split('\n')
        .filter(|line| {
            !line.starts_with('#')
                && !line.starts_with("$x")
                && !line.contains("$thoughts")
                && !line.is_empty()
        })
        .collect::<Vec<_>>()
        .join("\n")
        .replace("\\e", "\x1B");

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

    for replacer in replacers {
        chara_body = chara_body.replace(
            replacer.get("var").copied().unwrap(),
            replacer.get("val").copied().unwrap(),
        );
    }

    chara_body
}

pub fn format_character(messages: &str, chara: &String, max_width: usize, think: bool) -> String {
    let voice_line = if think { "o" } else { "╲" };

    let speech_bubble = create_speech_bubble(messages, max_width, think);
    let character = parse_character(chara, voice_line);

    format!("{}{}", speech_bubble, character)
}

pub fn list_chara() -> Vec<String> {
    Asset::iter()
        .map(|file| file.as_ref().replace(".chara", ""))
        .collect::<Vec<String>>()
}
