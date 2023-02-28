use std::io::{stdin, Read};

use charasay::{format_character, list_chara};
use clap::Parser;
use rand::seq::SliceRandom;
use textwrap::termwidth;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
struct Cli {
    /// Messages that chara want to say/think. If empty, read from STDIN.
    message: Vec<String>,

    /// Choose random chara.
    #[arg(short, long)]
    random: bool,

    /// Print all available chara.
    #[arg(short, long)]
    all: bool,

    /// Make chara only thinking about it, not saying it.
    #[arg(short, long)]
    think: bool,

    /// Max width of speech bubble. Default to terminal width.
    #[arg(short, long)]
    width: Option<usize>,

    /// Which chara should say/think
    #[arg(short = 'f', long = "file")]
    chara: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut messages = cli.message.join(" ");

    if messages.is_empty() {
        let mut buffer = String::new();

        stdin()
            .read_to_string(&mut buffer)
            .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

        messages = buffer.trim_end().to_string();
    }

    let max_width = match cli.width {
        Some(w) => w,
        None => termwidth() - 6,
    };

    if cli.all {
        let charas = list_chara();
        for chara in charas {
            println!("\n\n{}", chara);
            println!(
                "{}",
                format_character(messages.as_str(), &chara, max_width, cli.think)
            );
        }
    } else {
        let chara = if cli.random {
            let charas = list_chara();
            charas.choose(&mut rand::thread_rng()).unwrap().to_owned()
        } else {
            cli.chara.unwrap_or("cow".to_string())
        };

        println!(
            "{}",
            format_character(messages.as_str(), &chara, max_width, cli.think)
        );
    }
}
