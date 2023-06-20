mod utils;

use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: Provide text or file path");
        exit(1);
    }
    let msg_enc: String = encrypt_morse(&args[1]);
    println!("{:?}", msg_enc);
}

fn encrypt_morse(msg: &String) -> String {
    let msg_n: Vec<String> = msg
        .chars()
        .map(|x| x.to_uppercase().to_string())
        .collect();
    let mut converted_message: String = String::new();
    for m in msg_n {
        converted_message.push_str(utils::lookup_morse(&m));
        converted_message.push_str(" ");
    }
    converted_message
}
