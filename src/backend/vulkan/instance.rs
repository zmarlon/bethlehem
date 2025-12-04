use crate::backend::vulkan::{VulkanDevice, VulkanPhysicalDevice};
use crate::{Device, DeviceDesc, Error, InstanceDesc, PhysicalDevice, WindowHandle};
use ash::vk;
use libc::strcmp;
use std::ffi::CString;
use std::sync::Arc;

#[derive(Clone)]
pub struct VulkanInstance {
    pub(crate) entry: ash::Entry,
    pub(crate) instance: ash::Instance,
}

impl VulkanInstance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        let engine_name = CString::new(desc.engine_name)?;
        let application_name = CString::new(desc.application_name)?;

        let application_info = vk::ApplicationInfo::default()
            .api_version(vk::API_VERSION_1_3)
            .engine_name(engine_name.as_c_str())
            .application_name(application_name.as_c_str());

        let mut platform_extensions = vec![];

        let mut enabled_layer_names = vec![];

        if desc.debug {
            enabled_layer_names.push(c"VK_LAYER_KHRONOS_validation".as_ptr());
        }

        let mut enabled_extension_names = vec![];

        match desc.window_handle {
            WindowHandle::Sdl(sdl_window) => {
                platform_extensions = sdl_window
                    .vulkan_instance_extensions()?
                    .into_iter()
                    .map(|ext| Ok(CString::new(ext)?))
                    .collect::<Result<Vec<CString>, Error>>()?;
            }
        }

        for extension in &platform_extensions {
            enabled_extension_names.push(extension.as_c_str().as_ptr());
        }

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_layer_names(&enabled_layer_names)
            .enabled_extension_names(&enabled_extension_names);

        let entry = unsafe { ash::Entry::load()? };

        let instance = unsafe { entry.create_instance(&instance_create_info, None)? };

        Ok(Self { entry, instance })
    }

    pub fn get_physical_devices(self: &Arc<VulkanInstance>) -> Result<Vec<PhysicalDevice>, Error> {
        let physical_devices = unsafe { self.instance.enumerate_physical_devices()? }
            .into_iter()
            .map(|physical_device| {
                let extensions = unsafe {
                    self.instance
                        .enumerate_device_extension_properties(physical_device)?
                };

                let mut has_mesh_shader = false;

                for extension in &extensions {
                    if unsafe {
                        strcmp(
                            c"VK_EXT_mesh_shader".as_ptr(),
                            extension.extension_name.as_ptr(),
                        )
                    } == 0
                    {
                        has_mesh_shader = true;
                    }
                }

                let supported = has_mesh_shader;

                Ok((physical_device, supported))
            })
            .collect::<Result<Vec<_>, vk::Result>>()?;

        let physical_devices = physical_devices
            .into_iter()
            .filter(|(_, supported)| *supported)
            .map(|(physical_device, _)| {
                Ok(PhysicalDevice::Vulkan(Arc::new(VulkanPhysicalDevice::new(
                    self.clone(),
                    physical_device,
                )?)))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(physical_devices)
    }

    pub fn create_device(&self, desc: &DeviceDesc) -> Result<Device, Error> {
        let queue_create_infos = [];

        let extensions = [c"VK_KHR_swapchain".as_ptr(), c"VK_EXT_mesh_shader".as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .enabled_extension_names(&extensions)
            .queue_create_infos(&queue_create_infos);

        let device = unsafe {
            self.instance.create_device(
                desc.physical_device.as_vulkan_physical_device().handle,
                &device_create_info,
                None,
            )?
        };

        Ok(Device::Vulkan(Arc::new(VulkanDevice::new(device)?)))
    }
}
