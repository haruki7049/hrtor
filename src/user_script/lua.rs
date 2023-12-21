use crate::{file_loader::FileInfo, HrtorProcessor};

use super::UserScript;

pub struct LuaScript<'a> {
    pub(crate) hrtor: &'a HrtorProcessor,
    pub(crate) entrypoint: FileInfo,
}

impl UserScript for LuaScript<'_> {
    fn init(&self) {
        println!("Loading {}...", self.entrypoint.path);
    }
}
