pub enum CommandStatus {
    Continue(CommandResult),
    Quit,
}

pub enum CommandResult {
    Ok,
    NotFound(String),
    NothingToDo,
}
