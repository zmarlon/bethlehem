use crate::backend::metal::MetalDevice;
use crate::backend::metal::physical_device::MetalPhysicalDevice;
use crate::{Device, DeviceDesc, Error, InstanceDesc, PhysicalDevice};
use objc2_metal::*;
use std::sync::Arc;

pub struct MetalInstance {}

impl MetalInstance {
    pub fn new(desc: &InstanceDesc) -> Result<Self, Error> {
        Ok(Self {})
    }

    pub fn get_physical_devices(self: &Arc<Self>) -> Result<Vec<PhysicalDevice>, Error> {
        let devices = MTLCopyAllDevices();
        let devices = devices
            .iter()
            .filter(|device| device.supportsFamily(MTLGPUFamily::Metal4))
            .map(|device| {
                Ok(PhysicalDevice::Metal(Arc::new(MetalPhysicalDevice::new(
                    device,
                )?)))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(devices)
    }

    pub fn create_device(&self, desc: &DeviceDesc) -> Result<Device, Error> {
        let metal_physical_device = desc.physical_device.as_metal_physical_device().clone();

        Ok(Device::Metal(Arc::new(MetalDevice::new(
            metal_physical_device,
        )?)))
    }
}
