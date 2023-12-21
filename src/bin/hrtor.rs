use hrtor::{
    commands::io::{push_context, save_file},
    constants::PROMPT,
    file_loader::{get_config_info, get_file_info, FileInfo},
};

use linefeed::{Interface, ReadResult};
use rlua::Lua;
use std::error::Error;

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let mut file: FileInfo = get_file_info().unwrap();

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    // read config file
    let config: FileInfo = get_config_info().unwrap();

    // commands declaration
    let exit: String = String::from("exit");
    let print: String = String::from("print");
    let write: String = String::from("write");
    let add: String = String::from("add");
    let delete_all: String = String::from("delete_all");

    // lua_script
    let lua: Lua = Lua::new();
    lua.context(|lua_context| {
        // lua_script loading
        let _ = lua_context.load(&config.context).exec();
    });

    // mainloop by linefeed
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        // let input = input.parse::<Commands>().unwrap();
        match input {
            cmd if cmd == print => {
                println!("{}", &file.context);
            }
            cmd if cmd == write => {
                save_file(&file.path, &file.context);
            }
            cmd if cmd == add => {
                file.context = push_context();
            }
            cmd if cmd == delete_all => {
                file.context = String::new();
                println!("Deleted all in buffer's context");
            }
            cmd if cmd == exit => {
                break;
            }
            _ => {
                eprintln!("unknown command: {:?}", input);
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
