use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct CommandLineArguments {
    /// File's path
    #[arg(help = "The file you want to edit")]
    path: String,
}
