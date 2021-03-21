
#[derive(Debug)]
pub struct Size {
    width: u32,
    height: u32,
}

pub trait View : Drop {

    fn init(&self) -> ();

    fn resize(&self, size: Size) -> ();

    fn draw(&self, context: &crate::renderer::Context) -> ();
}
