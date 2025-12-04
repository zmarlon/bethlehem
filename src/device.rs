use std::sync::Arc;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;

pub enum Device {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanDevice>),
}
