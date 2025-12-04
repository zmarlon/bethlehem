use std::sync::Arc;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;
use crate::{Error, ShaderModule};
use crate::shader_module::ShaderDesc;

pub enum Device {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanDevice>),
}

impl Device {
    pub fn create_shader_module(&self, desc: &ShaderDesc) -> Result<ShaderModule, Error> {
        match self {
            #[cfg(feature = "vulkan")]
            Device::Vulkan(vulkan_device) => {
                vulkan_device.create_shader_module(desc)
            }
        }
    }
}