use crate::backend::vulkan::{VulkanPhysicalDevice, VulkanQueue, VulkanShaderModule};
use crate::shader_module::ShaderDesc;
use crate::{Error, Queue, ShaderModule, util};
use ash::vk;
use std::sync::Arc;

pub struct VulkanDevice {
    physical_device: Arc<VulkanPhysicalDevice>,

    pub(crate) device: ash::Device,

    direct_queue_family_index: u32,
    compute_queue_family_index: u32,
    transfer_queue_family_index: u32,

    direct_queue: Arc<VulkanQueue>,
    compute_queue: Arc<VulkanQueue>,
    transfer_queue: Arc<VulkanQueue>,
}

impl VulkanDevice {
    pub fn new(
        physical_device: Arc<VulkanPhysicalDevice>,
        device: ash::Device,

        direct_queue_family_index: u32,
        compute_queue_family_index: u32,
        transfer_queue_family_index: u32,
    ) -> Result<Self, Error> {
        let direct_queue = unsafe { device.get_device_queue(direct_queue_family_index, 0) };
        let compute_queue = unsafe { device.get_device_queue(compute_queue_family_index, 0) };
        let transfer_queue = unsafe { device.get_device_queue(transfer_queue_family_index, 0) };

        Ok(Self {
            physical_device,
            device,
            direct_queue_family_index,
            compute_queue_family_index,
            transfer_queue_family_index,
            direct_queue: Arc::new(VulkanQueue::new(direct_queue)),
            compute_queue: Arc::new(VulkanQueue::new(compute_queue)),
            transfer_queue: Arc::new(VulkanQueue::new(transfer_queue)),
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

    pub fn get_direct_queue(&self) -> Queue {
        Queue::Vulkan(self.direct_queue.clone())
    }

    pub fn get_compute_queue(&self) -> Queue {
        Queue::Vulkan(self.compute_queue.clone())
    }

    pub fn get_transfer_queue(&self) -> Queue {
        Queue::Vulkan(self.transfer_queue.clone())
    }
}
