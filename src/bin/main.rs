extern crate hrtor;
use hrtor::Commands;
use hrtor::PROMPT;
use hrtor::save_file;
use hrtor::push_context;
use hrtor::get_file_info;

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

    let config_map: HashMap<String, String> = read_config();
    apply_config(config_map);

    // mainloop by linefeed
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        let input = input.parse::<Commands>().unwrap();

        match input {
            Commands::Exit => {
                break;
            }
            Commands::Print => {
                println!("{}", file_context);
            }
            Commands::Write => {
                save_file(&filepath, &file_context);
            }
            Commands::Add => {
                file_context = push_context();
            }
            Commands::DeleteAll => {
                file_context = String::new();
                println!("Deleted all in buffer's context");
            }
        }
    }

    // Good bye message
    println!("Bye!!");
    Ok(())
}

struct Config {
    exit_name: String,
    print_name: String,
    write_name: String,
    add_name: String,
    delete_all_name: String,
}

fn read_config() -> HashMap<String, String> {
    // read config file
    let (_filepath, config_context) = get_file_info().unwrap();

    // lua_script
    let lua: Lua = Lua::new();
    let config_map: HashMap<String, String> = lua.context(|lua_context| {
        // lua_script loading
        let _ = lua_context.load(&config_context).exec();

        let mut config_map: HashMap<String, String> = HashMap::new();

        for pair in lua_context.globals().pairs::<String, String>() {
            let (key, value) = pair.unwrap();
            config_map.insert(key, value);
        }

        config_map
    });

    config_map
}

fn apply_config(config_map: HashMap<String, String>) {
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
