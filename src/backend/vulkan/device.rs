use crate::Error;
use crate::backend::vulkan::VulkanPhysicalDevice;
use std::sync::Arc;

pub struct VulkanDevice {
    physical_device: Arc<VulkanPhysicalDevice>,

    device: ash::Device,
}

impl VulkanDevice {
    pub fn new(
        physical_device: Arc<VulkanPhysicalDevice>,
        device: ash::Device,
    ) -> Result<Self, Error> {
        Ok(Self {
            physical_device,
            device,
        })
    }
}
