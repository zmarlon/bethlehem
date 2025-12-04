use crate::Error;
use crate::backend::vulkan::VulkanInstance;
use ash::vk;
use std::ffi::CStr;
use std::sync::Arc;

pub struct VulkanPhysicalDevice {
    instance: Arc<VulkanInstance>,

    pub(crate) physical_device: vk::PhysicalDevice,
    pub(crate) properties: vk::PhysicalDeviceProperties,

    pub(crate) name: String,
}

impl VulkanPhysicalDevice {
    pub fn new(
        instance: Arc<VulkanInstance>,
        physical_device: vk::PhysicalDevice,
    ) -> Result<Self, Error> {
        let properties = unsafe {
            instance
                .instance
                .get_physical_device_properties(physical_device)
        };

        let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_string_lossy()
            .to_string();

        Ok(Self {
            instance,
            physical_device,
            properties,
            name,
        })
    }
}
