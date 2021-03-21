use vulkano::{device::Queue, instance::{PhysicalDevice}, sync::SharingMode};
use vulkano::device::{Device, Features, DeviceOwned };
use winit::window::{ Window };
use vulkano::swapchain::{Swapchain, SurfaceTransform, PresentMode, ColorSpace, FullscreenExclusive};
use vulkano::image::ImageUsage;
use vulkano::framebuffer::{RenderPassAbstract};
use vulkano::format::Format;

use vulkano::pipeline::{GraphicsPipeline};
use vulkano::framebuffer::Subpass;
use std::sync::Arc;

/// Context.
pub struct Context {

    /// Logical device.
    device: Arc<Device>,

    /// Swapchain.
    swapchain: Arc<Swapchain<Window>>,

    /// Images of the swapchain.
    //images: Vec<Arc<SwapchainImage<Window>>>,

    /// Render pass.
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,

    /// Graphics queue.
    graphics_queue: Arc<Queue>,

    /// Transfer queue.
    transfer_queue: Arc<Queue>,
}

impl Context {

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
        let format = caps.supported_formats[0].0;

        // Create swapchain and images.
        let (swapchain, _images) = Swapchain::new(
            device.clone(), 
            window_context.surface.clone(),
            caps.min_image_count, 
            format, 
            dimensions, 
            1, 
            ImageUsage::color_attachment(), 
            SharingMode::Exclusive,
            SurfaceTransform::Identity,
            alpha, 
            PresentMode::Fifo, 
            FullscreenExclusive::Default,
            true, 
            ColorSpace::SrgbNonLinear)
            .expect("failed to create swapchain");

        // ----------------------------------------------------------------
        // Create render pass.

        let render_pass = vulkano::single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: Format::R8G8B8A8Unorm,
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

        mod vs {
            vulkano_shaders::shader!{
                ty: "vertex",
                src: "
#version 450
        
layout(location = 0) in vec2 position;
        
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}"
            }
        }
        
        mod fs {
            vulkano_shaders::shader!{
                ty: "fragment",
                src: "
#version 450
        
layout(location = 0) out vec4 f_color;
        
void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}"
            }
        }
        
        let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
        let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

        #[derive(Default, Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        
        vulkano::impl_vertex!(Vertex, position);

        let _graphics_pipeline = GraphicsPipeline::start()
            // Defines what kind of vertex input is expected.
            .vertex_input_single_buffer::<Vertex>()
            // The vertex shader.
            .vertex_shader(vs.main_entry_point(), ())
            // Defines the viewport (explanations below).
            .viewports_dynamic_scissors_irrelevant(1)
            // The fragment shader.
            .fragment_shader(fs.main_entry_point(), ())
            // This graphics pipeline object concerns the first pass of the render pass.
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            // Now that everything is specified, we call `build`.
            .build(device.clone())
            .unwrap();

        return Context
        {
            device,
            swapchain,
            //images,
            render_pass,
            graphics_queue: queues.next().expect("msg"),
            transfer_queue:queues.next().expect("msg"),
        }
    }

    pub fn swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self.swapchain
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
}

unsafe impl DeviceOwned for Context
{
    fn device(&self) -> &Arc<Device> {
        &self.device
    }
}