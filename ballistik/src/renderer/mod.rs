use vulkano::{device::QueuesIter, instance::{Instance, InstanceExtensions, PhysicalDevice}};
use vulkano::device::{ Device, Features };

use std::{io::Write, sync::Arc};

/// Context.
pub struct Context {

    /// Instance extensions.
    pub instance_extensions: InstanceExtensions,

    /// Instance.
    pub instance: Arc<Instance>,

    /// Logical device.
    pub device: Arc<Device>,

    /// Queues.
    pub queues: QueuesIter,
}

impl Context {

    pub fn new() -> Context {
        
        // Get instance extensions.
        let instance_extensions : InstanceExtensions = vulkano_win::required_extensions();

        // Create Vulkan instance.
        let instance = Instance::new(None, &instance_extensions, None)
            .expect("failed to create instance");

        // Select physical device.
        let physical_device = PhysicalDevice::enumerate(&instance)
            .next()
            .expect("no device available");

        // Select queue families.
        let queue_family = physical_device
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue family");

        // Create logical Vulkan device.
        let (device, mut queues) = 
        {
            // Setup device extensions.
            let device_extensions = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                .. vulkano::device::DeviceExtensions::none()
            };

            Device::new(
                physical_device, 
                &Features::none(), 
                &device_extensions,
                [(queue_family, 0.5)].iter().cloned())
                .expect("failed to create device")
        };

        let queue = queues.next();

        return Context
        {
            instance_extensions,
            instance,
            device,
            queues,
        }
    }
}