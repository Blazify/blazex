use crate::core::token::Token;
use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct FunDef {
    pub body_node: Nodes,
    pub name: Option<Token>,
    pub args: Vec<Token>,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl FunDef {
    pub fn new(body_node: Nodes, name: Option<Token>, args: Option<Vec<Token>>) -> FunDef {
        let pos_start: Position;
        let vec_args = args.unwrap_or(vec![]);
        if !name.clone().is_none() {
            pos_start = name.clone().unwrap().pos_start;
        } else if vec_args.len() > 0 {
            pos_start = vec_args[0].pos_start;
        } else {
            match body_node.clone() {
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
                Nodes::FunDef(node) => pos_start = node.pos_start,
                Nodes::BooleanNode(node) => pos_start = node.pos_start,
                Nodes::CallNode(node) => pos_start = node.pos_start,
            };
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
            Nodes::CallNode(node) => pos_end = node.pos_end,
            Nodes::FunDef(node) => pos_end = node.pos_end,
            Nodes::BooleanNode(node) => pos_end = node.pos_end,
        };

        FunDef {
            body_node,
            name,
            args: vec_args,
            pos_end,
            pos_start,
        }
    }
}
