
pub mod tree;
mod size;
mod rect;

pub mod test;

pub use self::size::Size;
pub use self::rect::Rect;

use crate::renderer::Context;
use crate::renderer::RenderContext;

///! View trait.
pub trait View {

    ///! Called when the view is initialized.
    fn init(&mut self, context: &Context) -> ();

    ///! Called when the view is dropped.
    fn drop(&mut self, context: &Context) -> ();

    ///! Called when the view has been resized.
    fn resize(&mut self, size: Size) -> ();

    ///! Called when the view should be redrawn.
    fn draw(&mut self, context: &Context, render_context: &RenderContext, rect: &Rect) -> ();
}
