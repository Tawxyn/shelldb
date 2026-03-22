use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        print!("db> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command = input.trim();
                println!("{}", command);
            }
            Err(error) => println!("{}", error),
        }
    }
}
