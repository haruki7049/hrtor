use std::sync::Arc;
use processor::HrtorProcessor;

pub struct Hrtor {
    pub processor: Arc<HrtorProcessor>,
}

impl Hrtor {
    pub fn new(processor: HrtorProcessor) -> Self {
        Self {
            processor: processor.into(),
        }
    }
}
