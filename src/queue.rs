#[cfg(feature = "metal")]
use crate::backend::metal::*;
use std::sync::Arc;

pub enum Queue {
    #[cfg(feature = "metal")]
    Metal(Arc<MetalQueue>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum QueueType {
    Direct,
    Compute,
    Transfer,
}
