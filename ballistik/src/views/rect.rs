

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Default for Rect {

    fn default() -> Self {
        Rect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

}

impl Rect {

    pub fn left(&self) -> u32 {
        self.left
    }

    pub fn left_mut(&mut self) -> &mut u32 {
        &mut self.left
    }

    pub fn top(&self) -> u32 {
        self.top
    } 

    pub fn top_mut(&mut self) -> &mut u32 {
        &mut self.top
    } 

    pub fn right(&self) -> u32 {
        self.right
    }

    pub fn right_mut(&mut self) -> &mut u32 {
        &mut self.right
    }

    pub fn bottom(&self) -> u32 {
        self.bottom
    }

    pub fn bottom_mut(&mut self) -> &mut u32 {
        &mut self.bottom
    }

}
