use crate::{Error, ShaderModule};
use crate::backend::vulkan::VulkanPhysicalDevice;
use std::sync::Arc;
use crate::shader_module::ShaderDesc;

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

    pub fn create_shader_module(&self, desc: &ShaderDesc) -> Result<ShaderModule, Error> {
        todo!()
    }
}
