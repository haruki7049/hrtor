use clap::Parser;
use linefeed::Interface;
use linefeed::ReadResult;
use std::error::Error;
use std::fs::read_to_string;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

/// PROMPT message in interpreter
const PROMPT: &str = "hrtor:> ";

/// command in interpreter, add.
const ADD: &str = "add";

/// command in interpreter, delete_all.
const DELETE_ALL: &str = "delete_all";

/// command in interpreter, print.
const PRINT: &str = "print";

/// command in interpreter, write.
const WRITE: &str = "write";

/// command in interpreter, exit.
const EXIT: &str = "exit";

/// CommandLine Argument
#[derive(Parser)]
struct AppArg {

    /// File's Path
    path: String,
}

/// read context from the file which is selected by CommandLine Argument
/// TODO: this function might be replaced by std::fs::File
fn read_context() -> Result<String, Box<dyn Error>> {
    let app = AppArg::parse();
    let context: String = read_to_string(app.path.as_str())?;
    Ok(context)
}

/// read filepath from the first argument
fn read_filepath() -> Result<String, Box<dyn Error>> {
    let app = AppArg::parse();
    let filepath: String = app.path;
    Ok(filepath)
}

fn main() -> Result<(), Box<dyn Error>> {
    // open file
    let mut f: File = File::open(read_filepath()?.as_str())?;
    // create a new buffer from above file
    let mut reader = BufReader::new(f);

    // get filepath
    let filepath: Path;

    // create interpreter by linefeed
    let reader = Interface::new(PROMPT).unwrap();
    reader.set_prompt(format!("{}", PROMPT).as_ref()).unwrap();

    println!("filepath: {}", filepath);

    // mainloop by linefeed
    while let ReadResult::Input(input) = reader.read_line().unwrap() {
        if input.eq(EXIT) {
            break;
        }
        if input.eq(PRINT) {
            println!("{}", buffer);
        }
        if input.eq(WRITE) {
            write_into_file(&mut buffer, &filepath);
        }
        if input.eq(ADD) {
            add_context_into_buffer(&mut buffer);
        }
        if input.eq(DELETE_ALL) {
            delete_all_context_in_buffer(&mut buffer);
        }
    }

    println!("Bye!!");
    Ok(())
}

#[cfg(test)]
mod test {
    //! tests for this main.rs
}
