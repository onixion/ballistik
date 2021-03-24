use super::super::View;

///! View node.
pub struct Node {

    ///! Dirty flag.
    dirty: bool,

    ///! View.
    view: Option<Box<dyn View>>,

    ///! Children.
    children: Vec<Node>,
}

impl Node {

    pub fn new(view: Box<dyn View>) -> Node {
        Node {
            dirty: true,
            view: Option::Some(view),
            children: Vec::<Node>::new(),
        }
    }

    pub fn dirty(&self) -> bool {
        self.dirty
    }

    pub fn view(&self) -> &Option<Box<dyn View>> {
        &self.view
    }

    pub fn view_mut(&mut self) -> &mut Option<Box<dyn View>> {
        &mut self.view
    }

    pub fn add_view(&mut self, view: Box<dyn View>) {

        let view_node = Node::new(view);
        self.children.push(view_node);
    }

    pub fn remove_view(&mut self, view: &Box<dyn View>) {
        
    }
}

impl Default for Node {

    fn default() -> Self {
        Node {
            dirty: true,
            view: Option::None,
            children: Vec::<Node>::new(),
        }
    }

}