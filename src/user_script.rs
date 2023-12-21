use crate::{file_loader::FileInfo, Hrtor};
pub mod lua;

pub struct UserScript<'a> {
    pub(crate) hrtor: &'a Hrtor<'a>,
    pub lua_entrypoint: FileInfo,
}
