use crate::core::token::Token;
use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct VarReassignNode {
    pub name: Token,
    pub value: Nodes,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl VarReassignNode {
    pub fn new(name: Token, value: Nodes) -> VarReassignNode {
        let pos_start: Position = name.clone().pos_start.clone();
        let pos_end: Position;
        match value.clone() {
            Nodes::Number(node) => pos_end = node.pos_end,
            Nodes::StringNode(node) => pos_end = node.pos_end,
            Nodes::CharNode(node) => pos_end = node.pos_end,
            Nodes::BinOp(node) => pos_end = node.pos_end,
            Nodes::IfNode(node) => pos_end = node.pos_end,
            Nodes::UnaryOp(node) => pos_end = node.pos_end,
            Nodes::ForNode(node) => pos_end = node.pos_end,
            Nodes::VarAssignNode(node) => pos_end = node.pos_end,
            Nodes::VarReassignNode(node) => pos_end = node.pos_end,
            Nodes::VarAccessNode(node) => pos_end = node.pos_end,
            Nodes::WhileNode(node) => pos_end = node.pos_end,
            Nodes::FunDef(node) => pos_end = node.pos_end,
            Nodes::CallNode(node) => pos_end = node.pos_end,
            Nodes::BooleanNode(node) => pos_end = node.pos_end,
        };

        VarReassignNode {
            name,
            value,
            pos_end,
            pos_start,
        }
    }
}
