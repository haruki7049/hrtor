/// get some context from standard input, and return String
#[cfg(windows)]
pub fn push_context() -> String {
    let mut inputed_text: String = String::new();
    loop {
        let mut last_line: String = String::new();

        std::io::stdin()
            .read_line(&mut last_line)
            .expect("failed to read line");

        if last_line.as_str() == ".\r\n" {
            break;
        }
        inputed_text.push_str(&last_line);
    }
    inputed_text
}

/// get some context from standard input, and return String
#[cfg(unix)]
pub fn push_context() -> String {
    let mut inputed_text: String = String::new();
    loop {
        let mut last_line: String = String::new();

        std::io::stdin()
            .read_line(&mut last_line)
            .expect("failed to read line");

        if last_line.as_str() == ".\n" {
            break;
        }
        inputed_text.push_str(&last_line);
    }
    inputed_text
}

/// save file
pub fn save_file(filepath: &String, file_context: &String) {
    if let Err(err) = std::fs::write(filepath, file_context) {
        eprintln!("Error saving file: {}", err);
    } else {
        println!("file saved successfully");
    }
}
