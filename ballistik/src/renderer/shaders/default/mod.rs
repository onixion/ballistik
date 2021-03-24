use vulkano::pipeline::{GraphicsPipeline};
use vulkano::device::Device;
use vulkano::framebuffer::Subpass;
use vulkano::framebuffer::{RenderPassAbstract};
use std::sync::Arc;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::descriptor::PipelineLayoutAbstract;

///! Vertex definition.
#[derive(Default, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position);

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

pub fn create(
    device: Arc<Device>, 
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>) 
    -> GraphicsPipeline<SingleBufferDefinition<Vertex>, Box<dyn PipelineLayoutAbstract + Send + Sync>, Arc<dyn RenderPassAbstract + Send + Sync>>  {

    let vs = vs::Shader::load(device.clone()).expect("failed to create shader module");
    let fs = fs::Shader::load(device.clone()).expect("failed to create shader module");

    GraphicsPipeline::start()
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
        .unwrap()
}
        
        