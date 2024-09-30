use processor::HrtorProcessor;
use std::sync::Arc;

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