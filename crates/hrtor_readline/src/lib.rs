use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::io::{Write, stdin, stdout};

#[derive(Default)]
pub struct Interpreter {
    pub config: Config,
}

pub struct Config {
    prompt: &'static str,
    interrupt: KeyEvent,
    eof: KeyEvent,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            prompt: "hrtor:> ",
            interrupt: KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
            eof: KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL),
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
        let mut stdout = std::io::stdout();
        let stdin = std::io::stdin();
        enable_raw_mode()?;

        stdout.write_all(self.config.prompt.as_bytes())?;
        stdout.flush()?;

        loop {
            match read()? {
                Event::Key(key_event) => match key_event {
                    // Interrupt
                    val if val == self.config.interrupt => {
                        disable_raw_mode()?;
                        return Err(Box::new(InterpreterError::KeyboardInterrupt));
                    }

                    // EOF
                    val if val == self.config.eof => {
                        disable_raw_mode()?;
                        return Ok(());
                    }

                    KeyEvent { code: KeyCode::Enter, .. } => {
                        stdout.write_all(b"\n")?;
                        stdout.flush()?;

                        continue;
                    }

                    _ => {}
                },
                _ => {}
            }
        }
    }
}

pub trait ReadLine {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
