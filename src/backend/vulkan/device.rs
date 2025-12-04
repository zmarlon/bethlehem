use crate::Error;

pub struct VulkanDevice {
    device: ash::Device,
}

impl VulkanDevice {
    pub fn new(device: ash::Device) -> Result<Self, Error> {
        Ok(Self { device })
    }
}
