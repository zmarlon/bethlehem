use std::sync::Arc;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;

#[cfg(feature = "metal")]
use crate::backend::metal::*;

use crate::shader_module::ShaderDesc;
use crate::{Error, ShaderModule};

pub enum Device {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanDevice>),

    #[cfg(feature = "metal")]
    Metal(Arc<MetalDevice>),
}

impl Device {
    pub fn create_shader_module(&self, desc: &ShaderDesc) -> Result<ShaderModule, Error> {
        match self {
            #[cfg(feature = "vulkan")]
            Device::Vulkan(vulkan_device) => vulkan_device.create_shader_module(desc),

            #[cfg(feature = "metal")]
            Device::Metal(metal_device) => metal_device.create_shader_module(desc),
        }
    }

    #[cfg(feature = "vulkan")]
    pub fn as_vulkan_device(&self) -> &Arc<VulkanDevice> {
        match self {
            Device::Vulkan(vulkan_device) => vulkan_device,
            _ => unreachable!(),
        }
    }

    #[cfg(feature = "metal")]
    pub fn as_metal_device(&self) -> &Arc<MetalDevice> {
        match self {
            Device::Metal(metal_device) => metal_device,
            _ => unreachable!(),
        }
    }
}
