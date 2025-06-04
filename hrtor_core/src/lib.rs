//! # hrtor_core

pub mod constants;
pub mod parser;

use constants::CommandStatus;
use std::path::PathBuf;
use thiserror::Error;

pub trait Loader {
    /// Loads the buffer
    fn buffer(&self) -> Result<FileInfo, LoadError>;
}

#[derive(Debug, Error)]
#[error("LOAD_ERROR: {message:?}")]
pub struct LoadError {
    pub message: String,
}

#[cfg(feature = "ropey")]
pub use ropey;

#[cfg(not(feature = "ropey"))]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
    pub context: String,
}

#[cfg(feature = "ropey")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub path: Option<PathBuf>,
    pub context: ropey::Rope,
}

pub enum ReadResult {
    Input(String),
    Signal(Signal),
    Eof,
}

pub enum Signal {
    Break,
    Continue,
    Interrupt,
    Resize,
    Suspend,
    Quit,
}

pub trait Processor {
    /// Handle the strings from inputs by main.rs on Hrtor implementation
    fn handle_command(&mut self, command: ReadResult) -> anyhow::Result<CommandStatus>;

    /// Evaluates the command
    fn eval(&mut self, str: String) -> anyhow::Result<CommandStatus>;
}
