use bzxc_shared::Node;

pub struct TypeChecker {
    pub node: Node,
}

impl TypeChecker {
    pub fn new(node: Node) -> Self {
        TypeChecker { node }
    }

    pub fn typed_node(&self) -> Node {
        self.node.clone()
    }
}
