use crate::Error;
use crate::backend::vulkan::VulkanDevice;
use ash::vk;
use std::sync::Arc;

pub struct VulkanShaderModule {
    device: Arc<VulkanDevice>,

    handle: vk::ShaderModule,
}

impl VulkanShaderModule {
    pub fn new(device: Arc<VulkanDevice>, handle: vk::ShaderModule) -> Result<Self, Error> {
        Ok(VulkanShaderModule { device, handle })
    }
}

impl Drop for VulkanShaderModule {
    fn drop(&mut self) {
        unsafe { self.device.device.destroy_shader_module(self.handle, None) };
    }
}
