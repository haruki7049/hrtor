//! Git action
//! TODO: Manage current directory's information by HrtorProcessor...?

use crate::processor::HrtorProcessor;
use crate::processor::constants::CommandStatus;
use std::process::Command;

impl HrtorProcessor {
    pub fn git(&self, arguments: &str) -> anyhow::Result<CommandStatus> {
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
