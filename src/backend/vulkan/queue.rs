use ash::vk;

pub struct VulkanQueue {
    handle: vk::Queue,
}

impl VulkanQueue {
    pub fn new(handle: vk::Queue) -> Self {
        Self { handle }
    }
}
