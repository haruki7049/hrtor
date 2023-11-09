extern crate hrtor;
use hrtor::Commands;
use hrtor::PROMPT;
use hrtor::save_file;
use hrtor::push_context;
use hrtor::get_file_info;
use hrtor::get_config_info;

use linefeed::Interface;
use linefeed::ReadResult;
use rlua::Lua;
use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

/// main function
fn main() -> Result<(), Box<dyn Error>> {
    let (filepath, mut file_context) = get_file_info().unwrap();

    println!("{}", &file_context);

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(PROMPT.to_string().as_ref()).unwrap();

    println!("filepath: {}", filepath);

    // read config file
    let (configpath, config_context) = get_config_info().unwrap();

    // commands declaration
    let mut exit: String = String::from("");
    let mut print: String = String::from("");
    let mut write: String = String::from("");
    let mut add: String = String::from("");
    let mut delete_all: String = String::from("");

    // lua_script
    let lua: Lua = Lua::new();
    let _ = lua.context(|lua_context| {
        // lua_script loading
        let _ = lua_context.load(&config_context).exec();

        let commands_table: rlua::Table = match lua_context.globals().get("commands") {
            Ok(table) => table,
            Err(_) => {
                eprintln!("cannot load commands' table in config file. you may not exit hrtor's command. YOU CAN USE CONTROL+D to exit.");
                return;
            }
        };

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
            ref_print if ref_print == print => {
                println!("{}", file_context);
            }
            ref_write if ref_write == write => {
                save_file(&filepath, &file_context);
            }
            ref_add if ref_add == add => {
                file_context = push_context();
            }
            ref_delete_all if ref_delete_all == delete_all => {
                file_context = String::new();
                println!("Deleted all in buffer's context");
            }
            ref_exit if ref_exit == exit => {
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
