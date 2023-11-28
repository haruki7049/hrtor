use hrtor::{
    commands::io::{push_context, save_file},
    constants::PROMPT,
    file_loader::{get_config_info, get_file_info, FileInfo},
};

use linefeed::{Interface, ReadResult};
use rlua::Lua;
use std::{
    error::Error,
};

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let mut file: FileInfo = get_file_info().unwrap();

    println!("{}", &file.context);

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    println!("filepath: {}", &file.path);

    // read config file
    let config: FileInfo = get_config_info().unwrap();

    // commands declaration
    let mut exit: String = String::from("");
    let mut print: String = String::from("");
    let mut write: String = String::from("");
    let mut add: String = String::from("");
    let mut delete_all: String = String::from("");

    // lua_script
    let lua: Lua = Lua::new();
    lua.context(|lua_context| {
        // lua_script loading
        let _ = lua_context.load(&config.context).exec();

        let commands_table: rlua::Table = lua_context.globals().get("commands").unwrap_or_else(|_| {
            eprintln!("cannot load commands' table in config file. you may not exit hrtor's command. YOU CAN USE CONTROL+D to exit.");
            lua_context.create_table().unwrap()
        });

        // loading each commands' alias
        exit = commands_table.get("exit").unwrap();
        print = commands_table.get("print").unwrap();
        write = commands_table.get("write").unwrap();
        add = commands_table.get("add").unwrap();
        delete_all = commands_table.get("delete_all").unwrap();

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
