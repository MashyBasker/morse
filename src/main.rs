mod utils;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Error: Follow the correct usage pattern");
        exit(1);
    } 
    if args[1] == "encrypt" {
        let msg_enc: Vec<String> = encrypt_morse(&args[2]);
        display_no_quotes(msg_enc);
    } else if args[1] == "encrypt-file" {
        display_file_morse(file_morse(&args[2]));
    }
    
}

fn encrypt_morse(msg: &String) -> Vec<String> {
    msg.chars()
        .map(|x| utils::lookup_morse(&x.to_uppercase().to_string()).to_string())
        .collect()
}

fn file_morse(filename: &String) -> Vec<Vec<String>> {
    let f_content = fs::read_to_string(filename).expect("Error: Could not read file");
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
