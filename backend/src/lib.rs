use hrtor_utils::FileInfo;
use std::sync::{Arc, Mutex};

pub struct Hrtor {
    pub editing_file: Arc<Mutex<FileInfo>>,
}
