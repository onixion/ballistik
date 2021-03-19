use vulkano::{device::QueuesIter, image::SwapchainImage, instance::{Instance, InstanceExtensions, PhysicalDevice}};
use vulkano::device::{ Device, Features };
use winit::window::{ Window };

use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, ColorSpace, FullscreenExclusive};
use vulkano::image::ImageUsage;

use std::sync::Arc;

use crate::renderer;
use crate::window;

/// Vulkan context.
pub struct Context {

    //swapchain: Arc<Swapchain<Window>>,
    //images: Vec<Arc<SwapchainImage<Window>>>,
}

impl Context {

    pub fn new(renderer_context: &renderer::Context, window_context: &window::Context) -> Context {
        
        // Select physical device.
        let physical_device = PhysicalDevice::enumerate(&renderer_context.instance)
            .next()
            .expect("no device available");

        let caps = window_context.surface.capabilities(physical_device)
            .expect("failed to get surface capabilities");

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;

        // let (swapchain, images) = Swapchain::new(
        //     renderer_context.device.clone(), 
        //     window_context.surface.clone(),
        //     caps.min_image_count, 
        //     format, 
        //     dimensions, 
        //     1, 
        //     ImageUsage::color_attachment(), 
        //     &renderer_context.queues,
        //     SurfaceTransform::Identity,
        //     alpha, 
        //     PresentMode::Fifo, 
        //     FullscreenExclusive::Default,
        //     true, 
        //     ColorSpace::SrgbNonLinear)
        //     .expect("failed to create swapchain");

        Context
        {
            //swapchain,
            //images,
        }
    }
}