use std::{
    io::{stdin, stdout, Read},
    path::PathBuf,
};

use charasay::{create_speech_bubble, format_character, list_chara};
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use rand::seq::SliceRandom;
use textwrap::termwidth;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "chara")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Make the character say something.
    Say {
        /// Messages that chara want to say/think. If empty, read from standard input.
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

        /// Which chara should say/think.
        #[arg(short = 'f', long = "file")]
        chara: Option<String>,
    },

    /// Generate completions for shell. Default to current shell.
    Completions {
        /// Shell syntax to use. Infer current shell when missing, fallback to bash.
        #[arg(short, long, value_enum)]
        shell: Option<Shell>,
    },

    /// List all built-in charas.
    List,

    /// TODO: Convert pixel-arts PNG to chara files.
    Convert {
        /// PNG file path.
        image: PathBuf,
    },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

fn main() {
    let cli = Cli::parse();

    // Run subcommands if match
    match cli.command {
        Commands::Say {
            message,
            random,
            all,
            think,
            width,
            chara,
        } => {
            let mut messages = message.join(" ");

            if messages.is_empty() {
                let mut buffer = String::new();

                stdin()
                    .read_to_string(&mut buffer)
                    .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

                messages = buffer.trim_end().to_string();
            }

            let max_width = match width {
                Some(w) => w,
                None => termwidth() - 6,
            };

            if all {
                let charas = list_chara();
                for chara in charas {
                    println!("\n\n{}", chara);
                    println!(
                        "{}",
                        format_character(messages.as_str(), &chara, max_width, think)
                    );
                }
            } else {
                let chara = if random {
                    let charas = list_chara();
                    charas.choose(&mut rand::thread_rng()).unwrap().to_owned()
                } else {
                    chara.unwrap_or("cow".to_string())
                };

                println!(
                    "{}",
                    format_character(messages.as_str(), &chara, max_width, think)
                );
            }
        }

        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let gen = match shell {
                Some(s) => s,
                None => Shell::from_env().unwrap_or(Shell::Bash),
            };

            print_completions(gen, &mut cmd);
        }

        Commands::List => {
            let charas = list_chara().join(" ");
            print!("{}", create_speech_bubble(&charas, termwidth() - 6, false))
        }

        Commands::Convert { image: _ } => todo!(),
    }
}
