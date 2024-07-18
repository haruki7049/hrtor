use std::sync::{Arc, Mutex};
use hrtor_utils::FileInfo;

pub struct Hrtor {
    pub editing_file: Arc<Mutex<FileInfo>>
}
