use std::env;
use std::process;

use chip8_binary_decoder::UserInput;

fn main() {
    let user_input = UserInput::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = chip8_binary_decoder::run(user_input) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
