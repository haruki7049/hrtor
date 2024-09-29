use clap::Parser;
use hrtor::{
    constants::PROMPT,
    CommandResult, CommandStatus, Hrtor, HrtorProcessor,
};
use file_loader::{ FileInfo, CommandLineArgsParser, };

use linefeed::Interface;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

#[derive(Parser)]
struct AppArg {
    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: String,

    //#[arg(long, default_value_t = String::from("./init.lua"))]
    #[arg(short, long, default_value_t = String::from("./init.lua"), help = "your config file which is as config.lua")]
    pub config: String,
}

impl CommandLineArgsParser for AppArg {
    fn read_fileinfo(&self) -> Result<FileInfo, Box<dyn Error>> {
        Ok(FileInfo {
            path: self.path.clone(),
            context: std::fs::read_to_string(self.path.clone()).unwrap_or_else(|_| {
                eprintln!("your file cannot find. create a new buffer to continue this process.");
                String::new()
            }),
        })
    }

    fn read_configinfo(&self) -> Result<FileInfo, Box<dyn Error>> {
        Ok(FileInfo {
            path: self.config.clone(),
            context: std::fs::read_to_string(self.config.clone()).unwrap_or_else(|_| {
                eprintln!("your config file cannot find. Continue this process without config file.");
                String::new()
            }),
        })
    }
}

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let app: AppArg = AppArg::parse();

    let file: FileInfo = app.read_fileinfo().unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    let mut instance = Hrtor::new(HrtorProcessor {
        editing_file: Arc::new(Mutex::new(file)),
    });

    // read config file
    if let Ok(config) = app.read_configinfo() {
        instance.load_luascript(config);
    }

    instance.init();

    // mainloop by linefeed
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
    println!("Bye!!");
    Ok(())
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
