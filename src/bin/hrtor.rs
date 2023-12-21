use hrtor::{
    constants::PROMPT,
    file_loader::{get_config_info, get_file_info, FileInfo},
    CommandResult, CommandStatus, Hrtor, HrtorProcessor,
};

use linefeed::Interface;
use std::{cell::RefCell, error::Error, rc::Rc};

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let file: FileInfo = get_file_info().unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    // read config file
    let config: FileInfo = get_config_info().unwrap();

    let mut processor = HrtorProcessor {
        editing_file: Rc::new(RefCell::new(file)),
    };

    let mut instance = Hrtor::new(&mut processor);
    instance.load_luascript(config);
    instance.init();

    // mainloop by linefeed
    while let CommandStatus::Continue(result) = {
        let read = reader.read_line().unwrap();
        instance.processor.handle_command(read)
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
