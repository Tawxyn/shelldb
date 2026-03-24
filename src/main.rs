use core::error;
use std::{
    error::Error,
    io::{self, Write},
};

#[derive(Debug)]
enum MetaCommand {
    exit,
}

#[derive(Debug)]
enum SqlCommand {
    insert,
    delete,
}

#[derive(Debug)]
enum InputType {
    Meta(MetaCommand),
    Sql(SqlCommand),
    Unknown,
}

fn check_input_type(input: &str) -> Result<InputType, InputType> {
    if input.starts_with('.') {
        match input {
            ".exit" => Ok(InputType::Meta(MetaCommand::exit)),
            _ => return Err(InputType::Unknown),
        }
    } else {
        match input {
            "insert" => Ok(InputType::Sql(SqlCommand::insert)),
            "delete" => Ok(InputType::Sql(SqlCommand::delete)),
            _ => return Err(InputType::Unknown),
        }
    }
}

fn main() {
    let mut input = String::new();
    loop {
        print!("db> ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let command = input.trim();
                let result = check_input_type(command);

                match result {
                    Ok(input_type) => println!("Valid command: {:?}", input_type),
                    Err(_) => println!("Unknown command"),
                }
            }
            Err(error) => println!("{}", error),
        }
        input.clear();
    }
}
