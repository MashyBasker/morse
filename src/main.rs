mod utils;

use clap::Parser;
use std::fs;
use std::io;
use std::io::Write;
use anyhow::Context;

#[derive(Parser)]
#[command(name = "morse")]
#[command(author = "Maharshi Basu <basumaharshi10@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Morse translator", long_about = None)]
struct Cli {
    #[arg(long)]
    mode: String,
    #[arg(long)]
    target: String,
}

fn main() {
    let cli = Cli::parse();
    if cli.mode == "text" {
        display_no_quotes(encrypt_morse(&cli.target));
    } else if cli.mode == "file" {
        display_file_morse(file_morse(&cli.target));
    } else {
        eprintln!("Error: Invalid arguments. Aborting");
    }
}

fn encrypt_morse(msg: &String) -> Vec<String> {
    msg.chars()
        .map(|x| utils::lookup_morse(&x.to_uppercase().to_string()).to_string())
        .collect()
}

fn file_morse(filename: &String) -> Vec<Vec<String>> {
    let f_content = fs::read_to_string(filename)
        .with_context(|| format!("Error: Could not read file `{}`", filename)).unwrap();
    let mut file_enc: Vec<Vec<String>> = Vec::new();
    for l in f_content.lines() {
        file_enc.push(encrypt_morse(&l.to_string()));
    }
    file_enc
}

fn display_file_morse(data: Vec<Vec<String>>) -> () {
    for line in data {
        display_no_quotes(line);
    }
}

fn display_no_quotes(text: Vec<String>) -> () {
    for t in text {
        print!("{} ", t);
        io::stdout().flush().unwrap();
    }
    println!("");
}
