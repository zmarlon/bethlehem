use crate::Error;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::MTL4CommandQueue;

pub struct MetalQueue {
    queue: Retained<ProtocolObject<dyn MTL4CommandQueue>>,
}

impl MetalQueue {
    pub fn new(queue: Retained<ProtocolObject<dyn MTL4CommandQueue>>) -> Result<Self, Error> {
        Ok(Self { queue })
    }
}
