use hrtor::{
    constants::PROMPT,
    file_loader::{get_config_info, get_file_info, FileInfo},
    CommandResult, CommandStatus, Hrtor, HrtorProcessor,
};

use linefeed::Interface;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let file: FileInfo = get_file_info().unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    let mut instance = Hrtor::new(HrtorProcessor {
        editing_file: Arc::new(Mutex::new(file)),
    });

    // read config file
    if let Some(config) = get_config_info() {
        instance.load_luascript(config);
    }

    instance.init();

    // mainloop by linefeed
    while let CommandStatus::Continue(result) = {
        let read = reader.read_line().unwrap();
        instance.processor.handle_command(read)
    } {
        match result {
            CommandResult::Ok => {}
            CommandResult::NothingToDo => {}
            CommandResult::NotFound(name) => {
                eprintln!("unknown command: {:?}", name);
            }
        }
    }
    println!("Bye!!");
    Ok(())
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
