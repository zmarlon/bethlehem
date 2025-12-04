mod backend;
mod instance;
mod physical_device;

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

    #[cfg(feature = "vulkan")]
    #[error("Vulkan loading error: {0}")]
    VulkanLoadingError(#[from] ash::LoadingError),

    #[cfg(feature = "vulkan")]
    #[error("Vulkan error: {0}")]
    VulkanError(#[from] ash::vk::Result),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BackendType {
    #[cfg(feature = "vulkan")]
    Vulkan,
}
