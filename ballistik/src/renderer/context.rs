
use std::sync::Arc;
use vulkano::device::Queue;
use vulkano::device::Device;

/// Vulkan context.
pub struct Context {
    /// Vulkan logical device.
    device: Arc<Device>,

    /// Graphics queue.
    graphics_queue: Arc<Queue>,

    /// Transfer queue.
    transfer_queue: Arc<Queue>,
}

impl Context {

    pub fn new(device: Arc<Device>, graphics_queue: Arc<Queue>, transfer_queue: Arc<Queue>) -> Context {
        Context 
        {
            device: device,
            graphics_queue: graphics_queue,
            transfer_queue: transfer_queue,
        }
    }

}