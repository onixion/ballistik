
#[derive(Debug, Clone, Copy)]
pub struct Size {
    width: u32,
    height: u32,
}

impl Default for Size {

    fn default() -> Self {
        Size {
            width: 0,
            height: 0,
        }
    }

}

