mod backend;
mod device;
mod instance;
mod physical_device;

pub use device::*;
pub use instance::*;
pub use physical_device::*;
use std::ffi::NulError;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("No backend found")]
    NoBackendFound,
    #[error("Nul error: {0}")]
    NulError(#[from] NulError),
    #[error("Unknown error")]
    Unknown,

    #[cfg(feature = "sdl")]
    #[error("Sdl error: {0}")]
    SdlError(#[from] sdl3::Error),

    #[cfg(feature = "vulkan")]
    #[error("Vulkan loading error: {0}")]
    VulkanLoadingError(#[from] ash::LoadingError),
    #[error("Vulkan Queue Family not found")]
    VulkanQueueFamilyNotFound,

    #[cfg(feature = "vulkan")]
    #[error("Vulkan error: {0}")]
    VulkanError(#[from] ash::vk::Result),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BackendType {
    #[cfg(feature = "vulkan")]
    Vulkan,
}
