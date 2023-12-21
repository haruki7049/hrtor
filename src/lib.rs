use file_loader::FileInfo;
use linefeed::ReadResult;
use user_script::UserScript;

mod actions;
pub mod constants;
pub mod file_loader;
pub mod user_script;

pub struct Hrtor {
    pub editing_file: FileInfo,
    pub user_script: UserScript,
}

pub enum CommandStatus {
    Continue(CommandResult),
    Quit,
}
pub enum CommandResult {
    Ok,
    NotFound(String),
}

impl Hrtor {
    pub fn handle_command(&mut self, command: ReadResult) -> CommandStatus {
        match command {
            ReadResult::Input(str) => {
                if str == "exit" {
                    return self.quit();
                }
                if str == "write" {
                    return self.write();
                }
                if str == "add" {
                    return self.add();
                }
                if str == "delete_all" {
                    return self.delete_all();
                }
                if str == "print" {
                    return self.print();
                }
                CommandStatus::Continue(CommandResult::NotFound(str))
            }
            _ => {
                eprintln!("Unexpected Result!");
                CommandStatus::Quit
            }
        }
    }
}
