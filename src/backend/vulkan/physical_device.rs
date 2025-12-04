use crate::Error;
use crate::backend::vulkan::VulkanInstance;
use ash::vk;
use std::ffi::CStr;
use std::sync::Arc;

pub struct VulkanPhysicalDevice {
    instance: Arc<VulkanInstance>,

    pub(crate) handle: vk::PhysicalDevice,
    pub(crate) properties: vk::PhysicalDeviceProperties,

    pub(crate) name: String,
}

impl VulkanPhysicalDevice {
    pub fn new(instance: Arc<VulkanInstance>, handle: vk::PhysicalDevice) -> Result<Self, Error> {
        let properties = unsafe { instance.instance.get_physical_device_properties(handle) };

        let name = unsafe { CStr::from_ptr(properties.device_name.as_ptr()) }
            .to_string_lossy()
            .to_string();

        Ok(Self {
            instance,
            handle,
            properties,
            name,
        })
    }
}
