pub mod views;
pub mod renderer;
pub mod renderer;

/// Ballistik system.
pub struct Ballistik {
    renderer: renderer::Renderer,
}

impl Ballistik {

    /// Create ballistik.
    pub fn new() -> Ballistik {
        Ballistik 
        {
            renderer: renderer::Renderer::new(),
        }
    }

    /// Run ballistik.
    pub fn run() -> () {
        
    }

}