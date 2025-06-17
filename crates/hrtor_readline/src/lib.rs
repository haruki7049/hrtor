use std::io::{Write, stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Default)]
pub struct Interpreter {
    pub config: Config,
}

pub struct Config {
    prompt: &'static str,
    interrupt: Key,
    eof: Key,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            prompt: "hrtor:> ",
            interrupt: Key::Ctrl('c'),
            eof: Key::Ctrl('d'),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InterpreterError {
    KeyboardInterrupt,
}

impl std::error::Error for InterpreterError {}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::KeyboardInterrupt => write!(f, "Keyboard interrupt detected"),
        }
    }
}

impl ReadLine for Interpreter {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let stdin = stdin();
        let mut stdout = stdout().into_raw_mode()?;

        stdout.write(&self.config.prompt.as_bytes())?;
        stdout.flush()?;

        for evt in stdin.events() {
            match evt? {
                Event::Key(key) => match key {
                    // Interrupt
                    val if val == self.config.interrupt => {
                        return Err(Box::new(InterpreterError::KeyboardInterrupt));
                    }

                    // EOF
                    val if val == self.config.eof => return Ok(()),

                    Key::Char('\n') => {
                        stdout.write(b"\n")?;
                        stdout.flush()?;

                        continue;
                    }

                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}

pub trait ReadLine {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
