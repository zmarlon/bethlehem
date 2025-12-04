use crate::{BackendType, Device, Error, PhysicalDevice};
use std::borrow::Cow;
use std::sync::Arc;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;

#[derive(Clone)]
pub enum Instance {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanInstance>),
}

impl Instance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        match desc.backend_type {
            #[cfg(feature = "vulkan")]
            BackendType::Vulkan => Ok(Instance::Vulkan(Arc::new(VulkanInstance::new(desc)?))),
        }
    }

    pub fn get_physical_devices(&self) -> Result<Vec<PhysicalDevice>, Error> {
        match self {
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(vulkan_instance) => vulkan_instance.get_physical_devices(),
        }
    }

    pub fn create_device(&self, desc: &DeviceDesc) -> Result<Device, Error> {
        match self {
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(vulkan_instance) => vulkan_instance.create_device(desc),
        }
    }

    pub fn backend(&self) -> BackendType {
        match self {
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(vulkan_instance) => BackendType::Vulkan,
        }
    }
}

pub struct InstanceDesc<'a> {
    pub backend_type: BackendType,
    pub debug: bool,
    pub engine_name: Cow<'a, str>,
    pub application_name: Cow<'a, str>,
    pub window_handle: WindowHandle<'a>,
}

pub enum WindowHandle<'a> {
    #[cfg(feature = "sdl")]
    Sdl(&'a sdl3::video::Window),
}

pub struct DeviceDesc<'a> {
    pub physical_device: &'a PhysicalDevice,
}
