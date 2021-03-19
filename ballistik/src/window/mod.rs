use winit::event_loop::EventLoop;
use winit::window::{ Window, WindowBuilder };

use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;

use std::sync::Arc;
use crate::renderer;

/// Window context.
pub struct Context {

    // Event loop.
    pub event_loop: EventLoop<()>,

    // Surface.
    pub surface: Arc<Surface<Window>>,
}

impl Context 
{
    pub fn new(context: &renderer::Context) -> Context {

        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, context.instance.clone())
            .unwrap();

        Context
        {
            event_loop: event_loop,
            surface: surface,
        }
    }
}

