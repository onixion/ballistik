
mod node;

pub use self::node::Node;

pub struct Tree {

    ///! Root node.
    root_node: Node,

}

impl Default for Tree {

    fn default() -> Tree {
        Tree {
            root_node: Node::default(),
        }
    }

}

impl Tree {

    pub fn root(&self) -> &Node {
        &self.root_node
    }

    pub fn root_mut(&mut self) -> &mut Node {
        &mut self.root_node
    }

}