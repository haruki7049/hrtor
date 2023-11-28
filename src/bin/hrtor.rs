use hrtor::{
    constants::PROMPT,
    file_loader::{get_config_info, get_file_info, FileInfo},
};

use linefeed::{Interface, ReadResult};
use rlua::{Function, Lua};
use std::{collections::HashMap, error::Error};

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    #[allow(unused_mut)]
    let mut file: FileInfo = get_file_info().unwrap();
    let config: FileInfo = get_config_info().unwrap();
    let commands: HashMap<String, Function> =
        gen_commands_map(&config.context).expect("failed to recognize config's commands...");
    let lua: Lua = Lua::new();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    // mainloop by linefeed
    loop {
        match reader.read_line() {
            Ok(ReadResult::Input(string)) => {
                execute_command(&string, &file, &commands, &lua);
            }
            Ok(ReadResult::Eof) => {
                return Ok(());
            }
            Err(err) => {
                eprintln!("failed to read text on this prompt by: {}", err);
            }
            Ok(ReadResult::Signal(_)) => {}
        }
    }
}

fn execute_command(
    _input: &str,
    mut _file: &FileInfo,
    _commands: &HashMap<String, Function>,
    _lua: &Lua,
) {
}

fn gen_commands_map(_config: &str) -> Result<HashMap<String, Function>, String> {
    Ok(HashMap::new())
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
