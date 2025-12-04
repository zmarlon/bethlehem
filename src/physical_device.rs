use std::sync::Arc;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;

pub enum PhysicalDevice {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanPhysicalDevice>),
}

impl PhysicalDevice {
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "vulkan")]
            PhysicalDevice::Vulkan(vulkan_physical_device) => &vulkan_physical_device.name,
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn as_vulkan_physical_device(&self) -> &Arc<VulkanPhysicalDevice> {
        match self {
            PhysicalDevice::Vulkan(vulkan_physical_device) => vulkan_physical_device,
            _ => unreachable!(),
        }
    }
}
