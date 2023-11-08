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

        let mut config_map: HashMap<String, String> = HashMap::new();

        let commands_table: rlua::Table = lua_context.globals().get("commands").expect("failed to get 'commands' table");

        // loading each commands' alias
        exit = commands_table.get("exit").expect("failed to get 'exit' variables");
        print = commands_table.get("print").expect("failed to get 'print' variables");
        write = commands_table.get("write").expect("failed to get 'write' variables");
        add = commands_table.get("add").expect("failed to get 'add' variables");
        delete_all = commands_table.get("delete_all").expect("failed to get 'delete_all' variables");
    });

    println!("{:?}, {:?}, {:?}, {:?}, {:?}", exit, print, write, add, delete_all);

    // mainloop by linefeed
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        // let input = input.parse::<Commands>().unwrap();
        println!("{:?}", input);

        match input {
            print => {
                println!("{}", file_context);
            }
            write => {
                save_file(&filepath, &file_context);
            }
            add => {
                file_context = push_context();
            }
            delete_all => {
                file_context = String::new();
                println!("Deleted all in buffer's context");
            }
            exit => {
                break;
            }
            _ => {
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
