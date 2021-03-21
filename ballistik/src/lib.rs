pub mod graphics;
pub mod window;
pub mod renderer;
pub mod views;

use views::View;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

/// Ballistik system.
pub struct Ballistik {
    pub renderer_context: renderer::Context,
    pub graphics_context: graphics::Context,
    pub window_context: window::Context,
}

impl Ballistik {

    /// Create ballistik.
    pub fn new() -> Self {

        let graphics_context = graphics::Context::new();
        let window_context = window::Context::new(&graphics_context);
        let renderer_context = renderer::Context::new(&graphics_context, &window_context);

        Ballistik 
        {
            graphics_context,
            renderer_context,
            window_context,
        }
    }

    pub fn add_view(&self, view: Box<dyn views::View>) {

        

    }

    /// Run ballistik.
    pub fn run(self) -> () {

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

    }
}