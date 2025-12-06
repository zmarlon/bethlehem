use crate::{BackendType, Device, Error, PhysicalDevice};
use std::borrow::Cow;
use std::sync::Arc;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;

#[cfg(feature = "metal")]
use crate::backend::metal::*;

#[derive(Clone)]
pub enum Instance {
    #[cfg(feature = "vulkan")]
    Vulkan(Arc<VulkanInstance>),

    #[cfg(feature = "metal")]
    Metal(Arc<MetalInstance>),
}

impl Instance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        match desc.backend_type {
            #[cfg(feature = "vulkan")]
            BackendType::Vulkan => Ok(Instance::Vulkan(Arc::new(VulkanInstance::new(desc)?))),

            #[cfg(feature = "metal")]
            BackendType::Metal => Ok(Instance::Metal(Arc::new(MetalInstance::new(desc)?))),
        }
    }

    pub fn get_physical_devices(&self) -> Result<Vec<PhysicalDevice>, Error> {
        match self {
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(vulkan_instance) => vulkan_instance.get_physical_devices(),

            #[cfg(feature = "metal")]
            Instance::Metal(metal_instance) => metal_instance.get_physical_devices(),
        }
    }

    pub fn create_device(&self, desc: &DeviceDesc) -> Result<Device, Error> {
        match self {
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(vulkan_instance) => vulkan_instance.create_device(desc),

            #[cfg(feature = "metal")]
            Instance::Metal(metal_instance) => metal_instance.create_device(desc),
        }
    }

    pub fn backend(&self) -> BackendType {
        match self {
            #[cfg(feature = "vulkan")]
            Instance::Vulkan(_) => BackendType::Vulkan,

            #[cfg(feature = "metal")]
            Instance::Metal(_) => BackendType::Metal,
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
