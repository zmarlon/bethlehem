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
        let vulkan_physical_device = desc.physical_device.as_vulkan_physical_device();

        let queue_family_properties = unsafe {
            self.instance
                .get_physical_device_queue_family_properties(vulkan_physical_device.handle)
        };

        //Queue families
        let (direct_queue_family_index, compute_queue_family_index, transfer_queue_family_index) =
            unsafe {
                find_queue_family_indices(
                    &self.instance,
                    vulkan_physical_device.handle,
                    &queue_family_properties,
                )
            }
            .ok_or_else(|| Error::VulkanQueueFamilyNotFound)?;

        let queue_priorities = [1.0];

        let mut device_queue_create_infos = vec![
            vk::DeviceQueueCreateInfo::default()
                .queue_family_index(direct_queue_family_index)
                .queue_priorities(&queue_priorities),
        ];

        if compute_queue_family_index != direct_queue_family_index {
            device_queue_create_infos.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(compute_queue_family_index)
                    .queue_priorities(&queue_priorities),
            );
        }

        if transfer_queue_family_index != direct_queue_family_index {
            device_queue_create_infos.push(
                vk::DeviceQueueCreateInfo::default()
                    .queue_family_index(transfer_queue_family_index)
                    .queue_priorities(&queue_priorities),
            );
        }

        let extensions = [c"VK_KHR_swapchain".as_ptr(), c"VK_EXT_mesh_shader".as_ptr()];

        let device_create_info = vk::DeviceCreateInfo::default()
            .enabled_extension_names(&extensions)
            .queue_create_infos(&device_queue_create_infos);

        let device = unsafe {
            self.instance
                .create_device(vulkan_physical_device.handle, &device_create_info, None)?
        };

        Ok(Device::Vulkan(Arc::new(VulkanDevice::new(device)?)))
    }
}

unsafe fn find_queue_family_indices(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    properties: &[vk::QueueFamilyProperties],
) -> Option<(u32, u32, u32)> {
    let direct_index = find_direct_queue_family_index(instance, physical_device, properties)?;
    let compute_index = find_queue_family_index(
        properties,
        vk::QueueFlags::COMPUTE,
        vk::QueueFlags::GRAPHICS | vk::QueueFlags::TRANSFER,
    )
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::COMPUTE,
            vk::QueueFlags::GRAPHICS,
        )
    })
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::COMPUTE,
            vk::QueueFlags::TRANSFER,
        )
    })
    .unwrap_or(direct_index);

    let transfer_index = find_queue_family_index(
        properties,
        vk::QueueFlags::TRANSFER,
        vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE,
    )
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::TRANSFER,
            vk::QueueFlags::GRAPHICS,
        )
    })
    .or_else(|| {
        find_queue_family_index(
            properties,
            vk::QueueFlags::TRANSFER,
            vk::QueueFlags::COMPUTE,
        )
    })
    .unwrap_or(direct_index);

    Some((direct_index, compute_index, transfer_index))
}

unsafe fn find_direct_queue_family_index(
    _instance: &ash::Instance,
    _physical_device: vk::PhysicalDevice,
    properties: &[vk::QueueFamilyProperties],
) -> Option<u32> {
    let mut queue_count: u32 = 0;
    let mut family_index: u32 = 0;

    let direct_flags: vk::QueueFlags =
        vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE | vk::QueueFlags::TRANSFER;

    for (i, properties) in properties.iter().enumerate() {
        let i = i as u32;

        if (properties.queue_flags & direct_flags) == direct_flags
            && properties.queue_count > queue_count
        {
            queue_count = properties.queue_count;
            family_index = i;
        }
    }

    if queue_count > 0 {
        Some(family_index)
    } else {
        None
    }
}

unsafe fn find_queue_family_index(
    properties: &[vk::QueueFamilyProperties],
    desired_flags: vk::QueueFlags,
    undesired_flags: vk::QueueFlags,
) -> Option<u32> {
    let mut queue_count: u32 = 0;
    let mut family_index: u32 = 0;

    for (i, properties) in properties.iter().enumerate() {
        let i = i as u32;

        if (properties.queue_flags & desired_flags) == desired_flags
            && (properties.queue_flags & undesired_flags) == vk::QueueFlags::empty()
            && properties.queue_count > queue_count
        {
            queue_count = properties.queue_count;
            family_index = i;
        }
    }

    if queue_count > 0 {
        Some(family_index)
    } else {
        None
    }
}
