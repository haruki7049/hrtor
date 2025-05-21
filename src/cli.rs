use clap::{CommandFactory, Parser};
use clap_complete::{Generator, Shell, generate};
use hrtor_core::FileInfo;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(version, about)]
/// CLI Arguments, parsed by clap crate.
pub struct CLIArgs {
    /// The file you want to edit
    #[clap(conflicts_with = "completion")]
    pub path: Option<PathBuf>,

    #[clap(long, group = "action", exclusive = true)]
    pub completion: Option<Shell>,
}

impl hrtor_core::Loader for CLIArgs {
    fn buffer(&self) -> FileInfo {
        let path: PathBuf = self.path.clone().unwrap_or_default();

        FileInfo {
            path: Some(path.clone()),
            context: std::fs::read_to_string(path).unwrap_or_else(|_| {
                eprintln!("your file cannot find. create a new buffer to continue this process.");
                String::new()
            }),
        }
    }
}

/// Print out the shell script for Hrtor completion
pub fn display_shellcompletion<G: Generator>(generator: G) {
    generate(
        generator,
        &mut CLIArgs::command(),
        env!("CARGO_PKG_NAME"),
        &mut std::io::stdout(),
    );
}

#[cfg(test)]
mod tests {
    use super::{CLIArgs, FileInfo};
    use hrtor_core::Loader;
    use std::path::PathBuf;

    #[test]
    /// How to read FileInfo from CLIArgs struct.
    fn how_to_read_buffer() -> anyhow::Result<()> {
        let args: CLIArgs = CLIArgs {
            path: Some(PathBuf::from("test.txt")),
            completion: None,
        };

        let fileinfo: FileInfo = args.buffer();

        assert_eq!(String::new(), fileinfo.context);

        Ok(())
    }
}
