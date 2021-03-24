pub mod graphics;
pub mod window;
pub mod renderer;
pub mod views;

use views::{View, tree};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

/// Ballistik system.
pub struct Ballistik {
    renderer_context: renderer::Context,
    graphics_context: graphics::Context,
    window_context: window::Context,

    views: Vec<Box<dyn View>>,
    //tree: views::tree::Tree,
}

impl Ballistik {

    /// Create ballistik.
    pub fn new() -> Self {

        let graphics_context = graphics::Context::new();
        let window_context = window::Context::new(&graphics_context);
        let renderer_context = renderer::Context::new(&graphics_context, &window_context);
        let tree = views::tree::Tree::default();

        Ballistik 
        {
            graphics_context,
            renderer_context,
            window_context,
            views: Vec::new(),
        }
    }

    pub fn add_view(&mut self, view: Box<dyn views::View>) {
        //self.tree.root_mut().add_view(view);
        self.views.push(view);
    }

    /// Run ballistik.
    pub fn run(self) -> () {

        let mut render_context = renderer::RenderContext::start(&self.renderer_context);

        render_context.end();

        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {

            println!("Event loop is running ...");

            self.window_context.event_loop.run(move |event, _, control_flow| {
    
                *control_flow = ControlFlow::Wait;
                
                match event {
                    Event::WindowEvent {
                        event: WindowEvent::CloseRequested,
                        window_id,
                    } => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            });
        });
    }
}