use clap::Parser;
use hrtor::{
    constants::PROMPT,
    file_loader::AppArg,
    file_loader::{get_config_info, get_file_info},
    CommandResult, CommandStatus, Hrtor, HrtorProcessor,
};
use hrtor_utils::FileInfo;

use linefeed::DefaultTerminal;
use linefeed::Interface;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let app: AppArg = AppArg::parse();

    let file: FileInfo = get_file_info(&app).unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    let mut instance = Hrtor::new(HrtorProcessor {
        editing_file: Arc::new(Mutex::new(file)),
    });

    // read config file
    if let Ok(config) = get_config_info(&app) {
        instance.load_luascript(config);
    }

    instance.init();

    ed_style(reader, instance);

    println!("Bye!!");
    Ok(())
}

/// mainloop by linefeed
fn ed_style(reader: Interface<DefaultTerminal>, instance: Hrtor) {
    while let CommandStatus::Continue(result) = {
        let read = reader.read_line().unwrap();
        instance.processor.handle_command(&instance, read)
    } {
        match result {
            CommandResult::Ok => {}
            CommandResult::NothingToDo => {}
            CommandResult::NotFound(name) => {
                eprintln!("unknown command: {:?}", name);
            }
        }
    }
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
