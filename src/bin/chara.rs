use std::{
    error::Error,
    io::{stdin, stdout, Read},
    path::PathBuf,
};
use charasay::errors::ReadInputError;

use charasay::{bubbles::BubbleType, format_character, print_character, Chara, BUILTIN_CHARA};
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

        /// Choose bubble type to use. Default to round.
        #[arg(short = 't', long, value_enum)]
        bubble_type: Option<BubbleType>,

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

fn print_all_characters(
    messages: &str,
    max_width: usize,
    bubble_type: BubbleType,
) -> Result<(), Box<dyn Error>> {
    let charas = BUILTIN_CHARA;
    for chara in charas {
        println!("\n\n{}", chara);
        println!(
            "{}",
            format_character(
                messages,
                &Chara::Builtin(chara.to_string()),
                max_width,
                bubble_type
            )?
        );
    }
    Ok(())
}

fn print_random_character(
    messages: &str,
    max_width: usize,
    bubble_type: BubbleType,
) -> Result<(), Box<dyn Error>> {
    let chara = Chara::Random;
    println!(
        "{}",
        format_character(messages, &chara, max_width, bubble_type)?
    );
    Ok(())
}

fn print_specified_character(
    messages: &str,
    chara_name: &str,
    max_width: usize,
    bubble_type: BubbleType,
) -> Result<(), Box<dyn Error>> {
    let chara = Chara::Builtin(chara_name.to_string());
    println!(
        "{}",
        format_character(messages, &chara, max_width, bubble_type)?
    );
    Ok(())
}

fn print_character_from_file(
    messages: &str,
    file_path: &str,
    max_width: usize,
    bubble_type: BubbleType,
) -> Result<(), Box<dyn Error>> {
    let chara = Chara::File(file_path.into());
    println!(
        "{}",
        format_character(messages, &chara, max_width, bubble_type)?
    );
    Ok(())
}

fn print_characters(
    charas: Charas,
    messages: String,
    max_width: usize,
    bubble_type: BubbleType,
) -> Result<(), Box<dyn Error>> {
    match charas {
        Charas { all: true, .. } => {
            // Print all built-in characters
            print_all_characters(&messages, max_width, bubble_type)?;
        }
        Charas { random: true, .. } => {
            // Print a random character
            print_random_character(&messages, max_width, bubble_type)?;
        }
        Charas { chara: Some(s), .. } => {
            // Print the specified character
            print_specified_character(&messages, &s, max_width, bubble_type)?;
        }
        Charas { file: Some(path), .. } => {
            // Print the character from a file
            print_character_from_file(&messages, path.to_str().unwrap(), max_width, bubble_type)?;
        }
        _ => {
            // Print the default character (cow)
            let chara = Chara::Builtin("cow".to_string());
            println!(
                "{}",
                format_character(&messages, &chara, max_width, bubble_type)?
            );
        }
    }
    Ok(())
}

fn read_input(message: Vec<String>) -> Result<String, ReadInputError> {
    let mut messages = message.join(" ");

    if messages.is_empty() {
        let mut buffer = String::new();

        if let Err(err) = stdin().read_to_string(&mut buffer) {
            return Err(ReadInputError::IoError(err));
        }

        messages = buffer.trim_end().to_string();
    }

    Ok(messages)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say {
            message,
            bubble_type,
            width,
            charas,
        } => {
            let messages = match read_input(message) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("Failed to read input: {:#?}", err);
                    std::process::exit(1);
                }
            };

            let max_width = width.unwrap_or(termwidth() - BORDER_WIDTH);
            let bubble_type = match bubble_type {
                Some(bt) => bt,
                None => BubbleType::Round,
            };

            print_characters(charas, messages, max_width, bubble_type)?;
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
    Ok(())
}
