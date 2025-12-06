use crate::backend::metal::MetalPhysicalDevice;
#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;
use std::sync::Arc;

pub enum PhysicalDevice {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanPhysicalDevice>),

    #[cfg(feature = "metal")]
    Metal(Arc<MetalPhysicalDevice>),
}

impl PhysicalDevice {
    pub fn name(&self) -> &str {
        match self {
            #[cfg(feature = "vulkan")]
            PhysicalDevice::Vulkan(vulkan_physical_device) => &vulkan_physical_device.name,

            #[cfg(feature = "metal")]
            PhysicalDevice::Metal(metal_physical_device) => &metal_physical_device.name,
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn as_vulkan_physical_device(&self) -> &Arc<VulkanPhysicalDevice> {
        match self {
            PhysicalDevice::Vulkan(vulkan_physical_device) => vulkan_physical_device,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "metal")]
    pub fn as_metal_physical_device(&self) -> &Arc<MetalPhysicalDevice> {
        match self {
            PhysicalDevice::Metal(metal_physical_device) => metal_physical_device,
            _ => unreachable!(),
        }
    }
}
