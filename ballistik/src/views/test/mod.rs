pub use super::View;
pub use super::Rect;
pub use super::Size;

pub struct TestView {


}

impl Default for TestView  {

    fn default() -> Self {
        TestView {

        }
    }

}

impl View for TestView {

    ///! Called when the view is initialized.
    fn init(&self, context: &crate::renderer::Context) -> () {

    }

    ///! Called when the view is dropped.
    fn drop(&self, context: &crate::renderer::Context) -> () {

    }

    ///! Called when the view has been resized.
    fn resize(&self, size: Size) -> () {

    }

    ///! Called when the view should be redrawn.
    fn draw(&self, rect: &Rect, context: &crate::renderer::Context) -> () {

    }

}