use crate::backend::vulkan::VulkanPhysicalDevice;
use crate::{Error, InstanceDesc, PhysicalDevice};
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

        let instance_create_info = vk::InstanceCreateInfo::default()
            .application_info(&application_info)
            .enabled_extension_names(&[])
            .enabled_layer_names(&[]);

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
}
