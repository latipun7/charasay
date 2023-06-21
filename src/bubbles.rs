use std::str::from_utf8;

use strip_ansi_escapes::strip;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

#[derive(Debug)]
pub enum BubbleType {
    Think,
    Round,
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

    pub fn create(self, messages: &str, max_width: &usize) -> String {
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
