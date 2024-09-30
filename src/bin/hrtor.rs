use clap::Parser;
use cli::by_clap::AppArg;
use cli::{ CLIArgs, CLI };
use constants::PROMPT;
use constants::{ CommandStatus, CommandResult };
use file_loader::{ FileInfo, CommandLineArgsParser };
use processor::{HrtorProcessor, Hrtor, Processor};

use linefeed::Interface;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let args: CLIArgs = AppArg::parse().eval().unwrap();

    let file: FileInfo = args.read_fileinfo().unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    let instance = Hrtor::new(HrtorProcessor {
        editing_file: Arc::new(Mutex::new(file)),
    });

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

    use cli::CLIArgs;
    use file_loader::{CommandLineArgsParser, FileInfo};

    #[test]
    fn how_to_use_apparg() {
        let args: CLIArgs = CLIArgs {
            text_file: String::from("test.txt"),
            config: String::from("config.lua"),
        };

        // Use below syntax
        // let args: CLIArgs = AppArg::parse().eval().unwrap();

        let fileinfo: FileInfo = args.read_fileinfo().unwrap();
        assert_eq!(fileinfo.path, "test.txt");
        assert_eq!(fileinfo.context, "");

        let configinfo: FileInfo = args.read_configinfo().unwrap();
        assert_eq!(configinfo.path, "config.lua");
        assert_eq!(configinfo.context, "");
    }
}
