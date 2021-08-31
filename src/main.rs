mod desugared;
mod errors;
mod nbe;
mod source;
mod syntax;
mod terms;

use std::io::Write;
use syntax::{parse_repl_input, ReplInput};

fn main() {
    (Lammy {}).start();
}

pub struct Lammy {}

impl Lammy {
    pub fn start(&mut self) {
        println!("Welcome to lammy v0.0.1");
        let mut input = String::new();
        loop {
            input.clear();
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();
            let result = parse_repl_input(&input);

            if result.errors.is_empty() {
                let should_continue = self.exec(result.result);
                if !should_continue {
                    break;
                }
            } else {
                for error in result.errors {
                    println!("{:?}", error);
                }
            }
        }
    }

    fn exec(&mut self, command: ReplInput) -> bool {
        println!("{:#?}", command);
        match command {
            ReplInput::Quit => false,
            _ => true,
        }
    }
}
