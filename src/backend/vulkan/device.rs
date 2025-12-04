use crate::backend::vulkan::{VulkanPhysicalDevice, VulkanShaderModule};
use crate::shader_module::ShaderDesc;
use crate::{Error, ShaderModule, util};
use ash::vk;
use std::sync::Arc;

pub struct VulkanDevice {
    physical_device: Arc<VulkanPhysicalDevice>,

    pub(crate) device: ash::Device,
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

    pub fn create_shader_module(
        self: &Arc<Self>,
        desc: &ShaderDesc,
    ) -> Result<ShaderModule, Error> {
        let spir_v = util::compile_hlsl(desc, true)?;

        let handle = unsafe {
            self.device.create_shader_module(
                &vk::ShaderModuleCreateInfo::default().code(bytemuck::cast_slice(&spir_v)),
                None,
            )
        }?;

        Ok(ShaderModule::Vulkan(VulkanShaderModule::new(
            self.clone(),
            handle,
        )?))
    }
}
