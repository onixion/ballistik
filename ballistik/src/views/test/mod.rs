pub use super::View;
pub use super::Rect;
pub use super::Size;

use crate::renderer::Context;
use vulkano::buffer::cpu_access::CpuAccessibleBuffer;
use crate::renderer::shaders::default::Vertex;
use vulkano::buffer::BufferUsage;
use vulkano::device::DeviceOwned;

use vulkano::memory::pool::PotentialDedicatedAllocation;
use vulkano::memory::pool::StdMemoryPoolAlloc;

use crate::renderer::RenderContext;

use std::sync::Arc;
use std::option::Option;

pub struct TestView {
    pub vertices: Option<Arc<CpuAccessibleBuffer<[Vertex], PotentialDedicatedAllocation<StdMemoryPoolAlloc>>>>,
}

impl Default for TestView  {

    fn default() -> Self {
        TestView {
            vertices: Option::None,
        }
    }

}

impl View for TestView {

    ///! Called when the view is initialized.
    fn init(&mut self, context: &Context) -> () {

        let vertex1 = Vertex { position: [-0.5, -0.5] };
        let vertex2 = Vertex { position: [ 0.0,  0.5] };
        let vertex3 = Vertex { position: [ 0.5, -0.25] };

        let vertices = vec![vertex1, vertex2, vertex3];

        self.vertices = Option::Some(CpuAccessibleBuffer::from_iter(
            context.device().clone(), 
            BufferUsage::all(), 
            false,
            vertices.into_iter()
        ).unwrap());

    }

    ///! Called when the view is dropped.
    fn drop(&mut self, context: &Context) -> () {


    }

    ///! Called when the view has been resized.
    fn resize(&mut self, size: Size) -> () {

    }

    ///! Called when the view should be redrawn.
    fn draw(&mut self, context: &Context, render_context: &RenderContext, rect: &Rect) -> () {

        use vulkano::command_buffer::DynamicState;
        use vulkano::pipeline::viewport::Viewport;

        let dynamic_state = DynamicState {
            viewports: Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [1024.0, 1024.0],
                depth_range: 0.0 .. 1.0,
            }]),
            .. DynamicState::none()
        };

        render_context.builder
            .draw(context.graphics_queue().clone(), &dynamic_state, self.vertices.clone(), (), (), ())
            .unwrap();

    
    }

}