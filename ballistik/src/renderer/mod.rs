use vulkano::{command_buffer::SubpassContents, device::Queue, instance::{PhysicalDevice}, sync::SharingMode};
use vulkano::device::{Device, Features, DeviceOwned };
use winit::window::{ Window };
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, ColorSpace, FullscreenExclusive};
use vulkano::image::ImageUsage;
use vulkano::framebuffer::{RenderPassAbstract};
use vulkano::format::Format;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::pool::standard::StandardCommandPoolBuilder;
use vulkano::framebuffer::Framebuffer;
use vulkano::swapchain;
use vulkano::image::swapchain::SwapchainImage;
use vulkano::swapchain::SwapchainAcquireFuture;
use vulkano::command_buffer::traits::CommandBuffer;

use vulkano::framebuffer::Subpass;
use std::sync::Arc;

pub mod shaders;
use shaders::default::Vertex;

/// Context.
pub struct Context {

    /// Logical device.
    device: Arc<Device>,

    /// Swapchain.
    swapchain: Arc<Swapchain<Window>>,

    /// Images of the swapchain.
    images: Vec<Arc<SwapchainImage<Window>>>,

    /// Render pass.
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,

    /// Graphics queue.
    graphics_queue: Arc<Queue>,

    /// Transfer queue.
    transfer_queue: Arc<Queue>,

    /// Graphics pipeline,
    default_pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<dyn PipelineLayoutAbstract + Send + Sync>, Arc<dyn RenderPassAbstract + Send + Sync>>>,
}

///! Render context.
impl Context {

    ///! New.
    pub fn new(
        graphics_context: &crate::graphics::Context, 
        window_context: &crate::window::Context) -> Self {
        
        // ----------------------------------------------------------------
        // Create device.

        // Select physical device.
       
        let physical_device = PhysicalDevice::enumerate(&graphics_context.instance)
            .next()
            .expect("no device available");

        // Select graphics queue.
        let graphics_queue = physical_device
            .queue_families()
            .find(|&q| q.supports_graphics())
            .expect("couldn't find a graphical queue");

        // Select transfer queue.
        let transfer_queue = physical_device
            .queue_families()
            .find(|&q| q.explicitly_supports_transfers())
            .expect("couldn't find a transfer queue");

        // Setup and create logical device.
        let (device, mut queues) = 
        {
            // Create device extensions.
            let device_extensions = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                .. vulkano::device::DeviceExtensions::none()
            };

            // Create logical device.
            Device::new(
                physical_device, 
                &Features::none(), 
                &device_extensions,
                [(graphics_queue, 0.5), (transfer_queue, 0.5)].iter().cloned())
                .expect("failed to create device")
        };

        // ----------------------------------------------------------------
        // Create swapchain.

        let caps = window_context.surface
            .capabilities(physical_device)
            .expect("failed to get surface capabilities");

        let dimensions = caps.current_extent.unwrap_or([1280, 1024]);
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[1];

        println!("################ {:?}", format);

        // Create swapchain and images.
        let (swapchain, images) = Swapchain::new(
            device.clone(), 
            window_context.surface.clone(),
            caps.min_image_count, 
            format.0, 
            dimensions, 
            1, 
            ImageUsage::color_attachment(), 
            SharingMode::Exclusive,
            SurfaceTransform::Identity,
            alpha, 
            PresentMode::Fifo, 
            FullscreenExclusive::Default,
            true, 
            format.1)
            .expect("failed to create swapchain");

        // ----------------------------------------------------------------
        // Create render pass.

        let render_pass = vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: Format::B8G8R8A8Srgb,
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap();

        let render_pass = Arc::new(render_pass) as Arc<dyn RenderPassAbstract + Send + Sync>;

        // ----------------------------------------------------------------
        // Create graphics pipeline.

        let default_pipeline = self::shaders::default::create(
            device.clone(),
            render_pass.clone()
        );

        return Context
        {
            device,
            swapchain,
            images,
            render_pass,
            graphics_queue: queues.next().expect("no graphics queue"),
            transfer_queue: queues.next().expect("no transfer queue"),
            default_pipeline: Arc::new(default_pipeline),
        }
    }

    pub fn swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self.swapchain
    }

    pub fn images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
        &self.images
    }

    pub fn render_pass(&self) -> &Arc<dyn RenderPassAbstract + Send + Sync> {
        &self.render_pass
    }

    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    pub fn transfer_queue(&self) -> &Arc<Queue> {
        &self.transfer_queue
    }

    pub fn default_pipeline(&self)-> &Arc<GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<dyn PipelineLayoutAbstract + Send + Sync>, Arc<dyn RenderPassAbstract + Send + Sync>>> {
        &self.default_pipeline
    }
}

unsafe impl DeviceOwned for Context
{
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}

pub struct RenderContext {
    pub builder: AutoCommandBufferBuilder<StandardCommandPoolBuilder>,
    acquire_future: SwapchainAcquireFuture<Window>,
}

impl RenderContext {

    pub fn start(context: &Context) -> RenderContext {

        let (image_num, b, acquire_future) = swapchain::acquire_next_image(
            context.swapchain().clone(), 
            None
        ).unwrap();

        let image = context.images()[image_num].clone();

        let framebuffer = Arc::new(
            Framebuffer::start(
                context.render_pass().clone()
            )
            .add(image.clone())
            .unwrap()
            .build()
            .unwrap()
        );

        let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
            context.device().clone(), 
            context.graphics_queue().family()
        ).unwrap();

        builder
            .begin_render_pass(
                framebuffer.clone(), 
                SubpassContents::Inline, 
                vec![[0.0, 0.0, 1.0, 1.0].into()])
            .unwrap();

        RenderContext {
            builder,
            acquire_future,
        }
    }

    pub fn end(&mut self) -> () {

        let mut command_buffer = self.builder
            .end_render_pass()
            .unwrap()
            .build()
            .unwrap();

        let finished = command_buffer.execute(self.graphics_queue().clone()).unwrap();
        finished.then_signal_fence_and_flush().unwrap().wait(None).unwrap();

    }

}
