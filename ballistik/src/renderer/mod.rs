use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use std::sync::Arc;
pub mod context;

pub struct Renderer
{
    instance: Arc<Instance>,
}

impl Renderer
{
    pub fn new() -> Renderer {

        let instance = Instance::new(None, &InstanceExtensions::none(), None)
            .expect("failed to create instance");

        let physical = PhysicalDevice::enumerate(&instance)
        .next().expect("no device available");

        let queue_family = physical
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue family");

        let (device, mut queues) = 
        {
            Device::new(
                physical, 
                &Features::none(), 
                &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned())
                .expect("failed to create device")
        };

        Renderer 
        {
            instance: instance,
        }
    }

}

