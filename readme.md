# `charasay 🐮`

[![Discord][discord-image]][discord-url]
[![GitHub Workflow Status][workflow-image]][workflow-url]

> **🐈 The future of cowsay 🐮! Colorful characters saying something 🗨️.**
>
> Re-engineered cowsay in rust 🦀. Display colorful ANSI arts saying something
> in your terminal 💻.

![Default character](https://user-images.githubusercontent.com/20012970/222370473-8a61c85f-7a14-49a4-a61f-44540d959286.png)

## Motivation

I use terminal emulator almost every day. I stare at it so much. I need some
entertainment in terminal, so I found [`ponysay`][ponysay] which is beautiful
and giving my terminal some colors. But `ponysay` kind of bloated for me since
I don't display all those ponies.

So, I want to make my own minimal tool to make my terminal so colorful and
display the character that I like. This chance is a great time to learn `rust`.
This project is mainly for me to learn rust and hopefully I get some feedback
while this make us all happy 😁.

## Installation

### AUR

For Arch Linux, package available via AUR. Example install this with AUR helper:

```console
yay -S charasay
```

or

```console
yay -S charasay-bin
```

### Cargo

If you have `rustup` or `cargo`, this tool available on crates.io. Install this with:

```console
cargo install charasay
```

### Manual

Just donwload from the [release page](https://github.com/latipun7/charasay/releases)
for your compatible Operating System, then extract the zip archive, give permission
to execute on extracted file, then place it on your `PATH`.

Alternatively, clone this repository, then build this with `cargo build --release`.

### Prerequisites

To display characters, your terminal needs to support true color (24-bit color).
Unicode fonts are needed to render the border of speech bubble.

## Usage

### Display Default Character to Say Something

Run `chara say something that motivating.` It would display colorful cow saying
`something that motivating.`.

If message is empty, it would accept from standard input, piping would works:
`fortune | chara say`.

### Display Different Character

Run `chara say -f ferris "Hello rustaceans!"`.

It could display external `.chara` files: `chara say -f ~/path/test.chara "Nice"`.

> Note: `.chara` files could be generated from PNG file.
>
> I want to implement this builtin in this tool. For now, you could generate
> `.cow` file with [Cowsay file converter][cowsay-converter] then rename `.cow`
> into `.chara`.

### Shell Completions

Shell completions also available with `chara completions` which would print out
completions script to standard output. Please consult to your shell documentation
on how to add completions.

### Consult to Help Command

For updated usage please consult to help command.

```console
$ chara --help
The future of cowsay 🐮! Colorful characters saying something 🗨️.

Usage: chara <COMMAND>

Commands:
say          Make the character say something
completions  Generate completions for shell. Default to current shell
convert      TODO: Convert pixel-arts PNG to chara files
help         Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```

```console
$ chara help say
Make the character say something

Usage: chara say [OPTIONS] [MESSAGE]...

Arguments:
[MESSAGE]...  Messages that chara want to say/think. If empty, read from STDIN

Options:
-r, --random         Choose random chara
-a, --all            Print all available chara
-t, --think          Make chara only thinking about it, not saying it
-w, --width <WIDTH>  Max width of speech bubble. Default to terminal width
-f, --file <CHARA>   Which chara should say/think
-h, --help           Print help
```

![Ferris](https://user-images.githubusercontent.com/20012970/222370485-3d43052f-977a-441e-a0c2-efc538d8e693.png)

## Hacking to the Gate~! 🧑‍💻🎶

[MIT License](./license) © Latif Sulistyo

### Acknowledgements

- All pixel-art artist on [each chara files](./src/charas/)
- [**@charc0al**][cowsay-converter] for cowsay file converter
- Rustaceans 🦀

<!-- Variables -->

[discord-image]: https://img.shields.io/discord/758271814153011201?label=Developers%20Indonesia&logo=discord&style=flat-square
[discord-url]: https://discord.gg/njSj2Nq "Chat and discuss at Developers Indonesia"
[workflow-image]: https://img.shields.io/github/actions/workflow/status/latipun7/charasay/ci-cd.yml?label=CI%2FCD&logo=github-actions&style=flat-square
[workflow-url]: https://github.com/latipun7/charasay/actions "GitHub Actions"
[ponysay]: https://github.com/erkin/ponysay "Ponysay GitHub Repository"
[cowsay-converter]: https://charc0al.github.io/cowsay-files/converter/ "Cowsay File Converter"
