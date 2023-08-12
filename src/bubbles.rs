use std::{error::Error, str::from_utf8};

use clap::ValueEnum;
use strip_ansi_escapes::strip;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, ValueEnum)]
pub enum BubbleType {
    Think,
    Round,
    Cowsay,
    Ascii,
    Unicode,
}

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

const COWSAY_BUBBLE: SpeechBubble = SpeechBubble {
    corner_top_left: " ",
    top: "_",
    corner_top_right: " \n",
    top_right: "  \\\n",
    right: "  |\n",
    bottom_right: "  /\n",
    corner_bottom_right: " \n",
    bottom: "-",
    corner_bottom_left: " ",
    bottom_left: "\\  ",
    left: "|  ",
    top_left: "/  ",
    short_left: "<  ",
    short_right: "  >\n",
};

const ASCII_BUBBLE: SpeechBubble = SpeechBubble {
    corner_top_left: " ",
    top: "_",
    corner_top_right: " \n",
    top_right: "  \\\n",
    right: "  |\n",
    bottom_right: "  |\n",
    corner_bottom_right: "/\n",
    bottom: "_",
    corner_bottom_left: "\\",
    bottom_left: "|  ",
    left: "|  ",
    top_left: "/  ",
    short_left: "/  ",
    short_right: "  \\\n",
};

const UNICODE_BUBBLE: SpeechBubble = SpeechBubble {
    corner_top_left: "┌",
    top: "─",
    corner_top_right: "┐\n",
    top_right: "  │\n",
    right: "  │\n",
    bottom_right: "  │\n",
    corner_bottom_right: "┘\n",
    bottom: "─",
    corner_bottom_left: "└",
    bottom_left: "│  ",
    left: "│  ",
    top_left: "│  ",
    short_left: "│  ",
    short_right: "  │\n",
};

#[derive(Debug)]
pub struct SpeechBubble {
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
    pub fn new(bubble_type: BubbleType) -> Self {
        match bubble_type {
            BubbleType::Think => THINK_BUBBLE,
            BubbleType::Round => ROUND_BUBBLE,
            BubbleType::Cowsay => COWSAY_BUBBLE,
            BubbleType::Ascii => ASCII_BUBBLE,
            BubbleType::Unicode => UNICODE_BUBBLE,
        }
    }

    fn line_len(line: &str) -> Result<usize, Box<dyn Error>> {
        let stripped = strip(line);
        let text = from_utf8(stripped.as_slice());

        Ok(text.map(UnicodeWidthStr::width).unwrap_or(0))
    }

    fn longest_line(lines: &[&str]) -> Result<usize, Box<dyn Error>> {
        let line_lengths = lines
            .iter()
            .map(|line| Self::line_len(line))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(line_lengths.into_iter().max().unwrap_or(0))
    }

    pub fn create(self, messages: &str, max_width: &usize) -> Result<String, Box<dyn Error>> {
        const SPACE: &str = " ";
        let wrapped = fill(messages, *max_width).replace('\t', "    ");
        let lines: Vec<&str> = wrapped.lines().collect();
        let line_count = lines.len();
        let actual_width = Self::longest_line(&lines)?;

        let total_size_buffer = (actual_width + 5) * 2 + line_count * (actual_width + 6);

        let mut write_buffer = Vec::with_capacity(total_size_buffer);

        // draw top box border
        write_buffer.push(self.corner_top_left);
        for _ in 0..(actual_width + 4) {
            write_buffer.push(self.top);
        }
        write_buffer.push(self.corner_top_right);

        // draw inner borders & messages
        for (i, line) in lines.into_iter().enumerate() {
            let left_border = match (line_count, i) {
                (1, _) => self.short_left,
                (_, 0) => self.top_left,
                (_, i) if i == line_count - 1 => self.bottom_left,
                _ => self.left,
            };
            write_buffer.push(left_border);

            let line_len = Self::line_len(line)?;
            write_buffer.push(line);
            write_buffer.resize(write_buffer.len() + actual_width - line_len, SPACE);

            let right_border = match (line_count, i) {
                (1, _) => self.short_right,
                (_, 0) => self.top_right,
                (_, i) if i == line_count - 1 => self.bottom_right,
                _ => self.right,
            };
            write_buffer.push(right_border);
        }

        // draw bottom box border
        write_buffer.push(self.corner_bottom_left);
        for _ in 0..(actual_width + 4) {
            write_buffer.push(self.bottom);
        }
        write_buffer.push(self.corner_bottom_right);

        Ok(write_buffer.join(""))
    }
}
