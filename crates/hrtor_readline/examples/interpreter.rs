use hrtor_readline::{Interpreter, ReadLine};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut interpreter: Interpreter = Interpreter::default();
    interpreter.run()?;

    Ok(())
}
