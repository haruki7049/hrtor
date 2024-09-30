/// PROMPT message in interpreter
pub const PROMPT: &str = "hrtor:> ";

pub enum CommandStatus {
    Continue(CommandResult),
    Quit,
}

pub enum CommandResult {
    Ok,
    NotFound(String),
    NothingToDo,
}
