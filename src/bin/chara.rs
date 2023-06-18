use std::{
    io::{stdin, stdout, Read},
    path::PathBuf,
};

use charasay::{format_character, print_character, Chara, BUILTIN_CHARA};
use clap::{Args, Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use textwrap::termwidth;

const BORDER_WIDTH: usize = 6;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about, name = "chara")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Make the character say something. Default to cow.
    Say {
        /// Messages that chara want to say/think. If empty, read from standard input.
        message: Vec<String>,

        /// Make chara only thinking about it, not saying it.
        #[arg(short, long)]
        think: bool,

        /// Max width of speech bubble. Default to terminal width.
        #[arg(short, long)]
        width: Option<usize>,

        #[command(flatten)]
        charas: Charas,
    },

    /// Generate completions for shell. Default to current shell.
    Completions {
        /// Shell syntax to use. Infer current shell when missing, fallback to bash.
        #[arg(short, long, value_enum)]
        shell: Option<Shell>,
    },

    /// List all built-in charas.
    List,

    /// Print only the character. Default to cow.
    Print {
        #[command(flatten)]
        charas: Charas,
    },

    /// TODO: Convert pixel-arts PNG to chara files.
    Convert {
        /// PNG file path.
        image: PathBuf,
    },
}

#[derive(Args, Debug)]
#[group(multiple = false)]
struct Charas {
    /// Choose built-in chara.
    #[arg(short, long, value_parser = BUILTIN_CHARA)]
    chara: Option<String>,

    /// Choose custom chara file.
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Choose random chara.
    #[arg(short, long)]
    random: bool,

    /// Print all built-in charas.
    #[arg(short, long)]
    all: bool,
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
}

fn print_characters(charas: Charas, messages: String, max_width: usize, think: bool) {
    if charas.all {
        print_all_characters(&messages, max_width, think);
    } else if charas.random {
        print_random_character(&messages, max_width, think);
    } else if let Some(s) = &charas.chara {
        print_specified_character(&messages, s, max_width, think);
    } else if let Some(path) = &charas.file {
        print_character_from_file(&messages, path.to_str().unwrap(), max_width, think);
    } else {
        let chara = Chara::Builtin("cow".to_string());
        println!("{}", format_character(&messages, &chara, max_width, think));
    }
}

fn print_all_characters(messages: &str, max_width: usize, think: bool) {
    let charas = BUILTIN_CHARA;
    for chara in charas {
        println!("\n\n{}", chara);
        println!(
            "{}",
            format_character(
                messages,
                &Chara::Builtin(chara.to_string()),
                max_width,
                think
            )
        );
    }
}

fn print_random_character(messages: &str, max_width: usize, think: bool) {
    let chara = Chara::Random;
    println!("{}", format_character(messages, &chara, max_width, think));
}

fn print_specified_character(messages: &str, chara_name: &str, max_width: usize, think: bool) {
    let chara = Chara::Builtin(chara_name.to_string());
    println!("{}", format_character(messages, &chara, max_width, think));
}

fn print_character_from_file(messages: &str, file_path: &str, max_width: usize, think: bool) {
    let chara = Chara::File(file_path.into());
    println!("{}", format_character(messages, &chara, max_width, think));
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say {
            message,
            think,
            width,
            charas,
        } => {
            let mut messages = message.join(" ");

            if messages.is_empty() {
                let mut buffer = String::new();

                stdin()
                    .read_to_string(&mut buffer)
                    .unwrap_or_else(|err| todo!("Log ERROR: {:#?}", err));

                messages = buffer.trim_end().to_string();
            }

            let max_width = width.unwrap_or(termwidth() - BORDER_WIDTH);

            print_characters(charas, messages, max_width, think);
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
            let charas = BUILTIN_CHARA.join(" ");
            println!("{}", charas)
        }

        Commands::Print { charas } => {
            let chara = match (charas.all, charas.random, charas.chara, charas.file) {
                (true, _, _, _) => Chara::All,
                (_, true, _, _) => Chara::Random,
                (_, _, Some(s), _) => Chara::Builtin(s),
                (_, _, _, Some(path)) => Chara::File(path),
                _ => Chara::Builtin("cow".to_string()),
            };

            println!("{}", print_character(&chara));
        }

        Commands::Convert { image: _ } => todo!(),
    }
}
