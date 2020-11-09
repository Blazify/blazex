use crate::core::token::Token;
use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct VarAssignNode {
    pub name: Token,
    pub value: Nodes,
    pub pos_start: Position,
    pub pos_end: Position,
    pub assignable: bool,
}

impl VarAssignNode {
    pub fn new(name: Token, value: Nodes, assignable: bool) -> VarAssignNode {
        let pos_start: Position = name.pos_start;
        let mut pos_end: Position = name.pos_end;
        match value.clone() {
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
            Nodes::FunDef(node) => pos_end = node.pos_end,
            Nodes::CallNode(node) => pos_end = node.pos_end,
            Nodes::BooleanNode(node) => pos_end = node.pos_end,
        };

        VarAssignNode {
            name,
            value,
            pos_end,
            pos_start,
            assignable,
        }
    }
}
