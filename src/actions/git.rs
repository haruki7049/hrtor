//! Git action
//! TODO: Manage current directory's information by HrtorProcessor...?

use crate::ProcessorImplementation;
use hrtor_core::constants::CommandStatus;
use std::process::Command;

pub trait HrtorGit {
    fn git(&self, arguments: &str) -> anyhow::Result<CommandStatus>;
}

impl HrtorGit for ProcessorImplementation {
    fn git(&self, arguments: &str) -> anyhow::Result<CommandStatus> {
        let stdout: std::io::Stdout = std::io::stdout();
        let stderr: std::io::Stderr = std::io::stderr();

        let _ = match arguments {
            "" => Command::new("git").stdout(stdout).stderr(stderr).output(),
            _ => Command::new("git")
                .stdout(stdout)
                .stderr(stderr)
                .arg(arguments)
                .output(),
        };

        Ok(CommandStatus::Continue)
    }
}
