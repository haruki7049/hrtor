#[derive(Debug)]
pub enum CommandStatus {
    Continue(CommandResult),
    Quit,
}

#[derive(Debug)]
pub enum CommandResult {
    Ok,
    NotFound(String),
    NothingToDo,
}
