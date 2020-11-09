use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct WhileNode {
    pub condition_node: Nodes,
    pub body_node: Nodes,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl WhileNode {
    pub fn new(condition_node: Nodes, body_node: Nodes) -> WhileNode {
        let pos_start: Position;
        match condition_node.clone() {
            Nodes::Number(node) => pos_start = node.pos_start,
            Nodes::StringNode(node) => pos_start = node.pos_start,
            Nodes::CharNode(node) => pos_start = node.pos_start,
            Nodes::BinOp(node) => pos_start = node.pos_start,
            Nodes::IfNode(node) => pos_start = node.pos_start,
            Nodes::UnaryOp(node) => pos_start = node.pos_start,
            Nodes::ForNode(node) => pos_start = node.pos_start,
            Nodes::VarAssignNode(node) => pos_start = node.pos_start,
            Nodes::VarAccessNode(node) => pos_start = node.pos_start,
            Nodes::WhileNode(node) => pos_start = node.pos_start,
            Nodes::BooleanNode(node) => pos_start = node.pos_start,
            Nodes::FunDef(node) => pos_start = node.pos_start,
            Nodes::CallNode(node) => pos_start = node.pos_start,
        };

        let pos_end: Position;
        match body_node.clone() {
            Nodes::Number(node) => pos_end = node.pos_end,
            Nodes::StringNode(node) => pos_end = node.pos_end,
            Nodes::CharNode(node) => pos_end = node.pos_end,
            Nodes::BinOp(node) => pos_end = node.pos_end,
            Nodes::IfNode(node) => pos_end = node.pos_end,
            Nodes::UnaryOp(node) => pos_end = node.pos_end,
            Nodes::ForNode(node) => pos_end = node.pos_end,
            Nodes::VarAssignNode(node) => pos_end = node.pos_end,
            Nodes::VarAccessNode(node) => pos_end = node.pos_end,
            Nodes::WhileNode(node) => pos_end = node.pos_end,
            Nodes::BooleanNode(node) => pos_end = node.pos_end,
            Nodes::FunDef(node) => pos_end = node.pos_end,
            Nodes::CallNode(node) => pos_end = node.pos_end,
        };

        WhileNode {
            pos_start,
            pos_end,
            condition_node,
            body_node,
        }
    }
}
