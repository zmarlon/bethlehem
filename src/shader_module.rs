use std::borrow::Cow;

#[cfg(feature = "vulkan")]
use crate::backend::vulkan::*;

#[cfg(feature = "metal")]
use crate::backend::metal::*;

pub enum ShaderModule {
    #[cfg(feature = "vulkan")]
    Vulkan(VulkanShaderModule),

    #[cfg(feature = "metal")]
    Metal(MetalShaderModule),
}

pub struct ShaderDesc {
    pub name: Cow<'static, str>,
    pub source: ShaderSource,
    pub kind: ShaderKind,
    pub entry_point: Cow<'static, str>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ShaderKind {
    Vertex,
    Task,
    Mesh,
    Fragment,
    Compute,
}

#[derive(Clone)]
pub enum ShaderSource {
    Hlsl {
        source: Cow<'static, str>,
        defines: Vec<(Cow<'static, str>, Option<Cow<'static, str>>)>,
    },
}
