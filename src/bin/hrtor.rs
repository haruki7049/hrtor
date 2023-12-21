use hrtor::{
    constants::PROMPT,
    file_loader::{get_config_info, get_file_info, FileInfo},
    user_script::UserScript,
    CommandResult, CommandStatus, Hrtor,
};

use linefeed::Interface;
use std::error::Error;

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let file: FileInfo = get_file_info().unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    // read config file
    let config: FileInfo = get_config_info().unwrap();

    let mut instance: Hrtor<'a> = Hrtor {
        editing_file: file,
        user_script: UserScript {
            hrtor: 
            lua_entrypoint: config,
        },
    };

    // mainloop by linefeed
    while let CommandStatus::Continue(result) = {
        let read = reader.read_line().unwrap();
        instance.handle_command(read)
    } {
        match result {
            CommandResult::Ok => {}
            CommandResult::NotFound(name) => {
                eprintln!("unknown command: {:?}", name);
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
