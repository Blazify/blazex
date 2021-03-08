use crate::core::token::Token;
use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct UnaryNode {
    pub node: Nodes,
    pub op_token: Token,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl UnaryNode {
    pub fn new(node: Nodes, op_token: Token) -> UnaryNode {
        let pos_start: Position;
        let pos_end: Position;
        match node.clone() {
            Nodes::Number(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::StringNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::CharNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::BinOp(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::IfNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::UnaryOp(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::ForNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::VarAssignNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::VarReassignNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::VarAccessNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::WhileNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::FunDef(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::CallNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
            Nodes::BooleanNode(node) => {
                pos_start = node.pos_start;
                pos_end = node.pos_end
            }
        };

        UnaryNode {
            node,
            op_token,
            pos_start,
            pos_end,
        }
    }
}
