
pub mod tree;
mod size;
mod rect;

pub mod test;

pub use self::size::Size;
pub use self::rect::Rect;

///! View trait.
pub trait View {

    ///! Called when the view is initialized.
    fn init(&self, context: &crate::renderer::Context) -> ();

    ///! Called when the view is dropped.
    fn drop(&self, context: &crate::renderer::Context) -> ();

    ///! Called when the view has been resized.
    fn resize(&self, size: Size) -> ();

    ///! Called when the view should be redrawn.
    fn draw(&self, rect: &Rect, context: &crate::renderer::Context) -> ();
}
