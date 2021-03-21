use vulkano::instance::{Instance, InstanceExtensions};
use std::sync::Arc;

/// Context.
pub struct Context {

    /// Instance extensions.
    pub instance_extensions: InstanceExtensions,

    /// Instance.
    pub instance: Arc<Instance>,
}

impl Context {

    pub fn new() -> Self {
        
        // Get instance extensions.
        let instance_extensions : InstanceExtensions = vulkano_win::required_extensions();

        // Create Vulkan instance.
        let instance = Instance::new(None, &instance_extensions, None)
            .expect("failed to create instance");

        return Context
        {
            instance_extensions,
            instance,
        }
    }
}