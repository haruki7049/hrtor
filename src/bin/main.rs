extern crate hrtor;
use hrtor::Commands;
use hrtor::PROMPT;
use hrtor::save_file;
use hrtor::push_context;
use hrtor::get_file_info;

use linefeed::Interface;
use linefeed::ReadResult;
use std::error::Error;

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let (filepath, mut file_context) = get_file_info().unwrap();

    println!("{}", &file_context);

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    println!("filepath: {}", filepath);

    // mainloop by linefeed
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        let input = input.parse::<Commands>().unwrap();

        match input {
            Commands::Exit => {
                break;
            }
            Commands::Print => {
                println!("{}", file_context);
            }
            Commands::Write => {
                save_file(&filepath, &file_context);
            }
            Commands::Add => {
                file_context = push_context();
            }
            Commands::DeleteAll => {
                file_context = String::new();
                println!("Deleted all in buffer's context");
            }
        }
    }

    // Good bye message
    println!("Bye!!");
    Ok(())
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
