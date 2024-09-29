use std::error::Error;

pub trait CLI {
    fn eval(&self) -> Result<CLIArgs, Box<dyn Error>>;
}

pub struct CLIArgs {
    pub text_file: String,
    pub config: String,
}
