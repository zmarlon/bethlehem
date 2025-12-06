use crate::Error;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::*;

pub struct MetalPhysicalDevice {
    pub(crate) mtl_device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub(crate) name: String,
}

impl MetalPhysicalDevice {
    pub fn new(mtl_device: Retained<ProtocolObject<dyn MTLDevice>>) -> Result<Self, Error> {
        let name = mtl_device.name().to_string();

        Ok(Self { mtl_device, name })
    }
}
