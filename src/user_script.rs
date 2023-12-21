use crate::file_loader::FileInfo;
pub mod lua;

pub struct UserScript {
    pub lua_entrypoint: FileInfo,
}
