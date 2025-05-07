use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: PathBuf,
    pub context: String,
}

#[derive(Parser)]
pub struct CLIArgs {
    /// File's Path
    #[arg(help = "The file you want to edit")]
    pub path: PathBuf,
}

impl CLIArgs {
    pub fn read_fileinfo(&self) -> FileInfo {
        FileInfo {
            path: self.path.clone(),
            context: std::fs::read_to_string(self.path.clone()).unwrap_or_else(|_| {
                eprintln!("your file cannot find. create a new buffer to continue this process.");
                String::new()
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CLIArgs;
    use super::FileInfo;

    #[test]
    /// How to read FileInfo from CLIArgs struct.
    fn how_to_read_fileinfo() -> anyhow::Result<()> {
        let args: CLIArgs = CLIArgs {
            path: String::from("test.txt"),
        };

        let fileinfo: FileInfo = args.read_fileinfo();

        assert_eq!(String::new(), fileinfo.context);

        Ok(())
    }
}
