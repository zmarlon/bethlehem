use crate::Error;
use metal_irconverter::sys::IRResourceLocation;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_metal::{MTLFunction, MTLLibrary};

pub struct MetalShaderModule {
    library: Retained<ProtocolObject<dyn MTLLibrary>>,
    function: Retained<ProtocolObject<dyn MTLFunction>>,
    locations: Vec<IRResourceLocation>,
}
impl MetalShaderModule {
    pub fn new(
        library: Retained<ProtocolObject<dyn MTLLibrary>>,
        function: Retained<ProtocolObject<dyn MTLFunction>>,
        locations: Vec<IRResourceLocation>,
    ) -> Result<Self, Error> {
        Ok(Self {
            library,
            function,
            locations,
        })
    }
}
