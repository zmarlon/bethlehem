#![feature(str_as_str)]

mod backend;
mod device;
mod instance;
mod physical_device;
mod shader_module;
mod util;

pub use device::*;
pub use instance::*;
pub use physical_device::*;
pub use shader_module::*;

use hassle_rs::HassleError;
use std::ffi::NulError;
use thiserror::Error;

#[cfg(feature = "metal")]
use objc2::rc::Retained;
use objc2_foundation::NSError;

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

    #[error("Invalid shader source")]
    InvalidShaderSource,

    #[error("Hassle rs error: {0}")]
    HassleRs(#[from] HassleError),

    #[cfg(feature = "metal")]
    #[error("Metal shader converter error: {0}")]
    MetalShaderConverter(String),

    #[cfg(feature = "metal")]
    #[error("NS error")]
    NSError(#[from] Retained<NSError>),

    #[cfg(feature = "metal")]
    #[error("Metal error")]
    MetalError(String),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BackendType {
    #[cfg(feature = "vulkan")]
    Vulkan,
    #[cfg(feature = "metal")]
    Metal,
}
