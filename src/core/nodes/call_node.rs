use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct CallNode {
    pub node_to_call: Nodes,
    pub args: Vec<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl CallNode {
    pub fn new(node_to_call: Nodes, args: Option<Vec<Nodes>>) -> CallNode {
        let vec_args = args.unwrap_or(vec![]);

        let pos_start: Position;
        if vec_args.len() > 0 {
            match vec_args[0].clone() {
                Nodes::Number(node) => pos_start = node.pos_start,
                Nodes::StringNode(node) => pos_start = node.pos_start,
                Nodes::CharNode(node) => pos_start = node.pos_start,
                Nodes::BinOp(node) => pos_start = node.pos_start,
                Nodes::IfNode(node) => pos_start = node.pos_start,
                Nodes::UnaryOp(node) => pos_start = node.pos_start,
                Nodes::ForNode(node) => pos_start = node.pos_start,
                Nodes::VarAssignNode(node) => pos_start = node.pos_start,
                Nodes::VarReassignNode(node) => pos_start = node.pos_start,
                Nodes::VarAccessNode(node) => pos_start = node.pos_start,
                Nodes::WhileNode(node) => pos_start = node.pos_start,
                Nodes::CallNode(node) => pos_start = node.pos_start,
                Nodes::FunDef(node) => pos_start = node.pos_start,
                Nodes::BooleanNode(node) => pos_start = node.pos_start,
            };
        } else {
            match node_to_call.clone() {
                Nodes::Number(node) => pos_start = node.pos_start,
                Nodes::StringNode(node) => pos_start = node.pos_start,
                Nodes::CharNode(node) => pos_start = node.pos_start,
                Nodes::BinOp(node) => pos_start = node.pos_start,
                Nodes::IfNode(node) => pos_start = node.pos_start,
                Nodes::UnaryOp(node) => pos_start = node.pos_start,
                Nodes::ForNode(node) => pos_start = node.pos_start,
                Nodes::VarAssignNode(node) => pos_start = node.pos_start,
                Nodes::VarReassignNode(node) => pos_start = node.pos_start,
                Nodes::VarAccessNode(node) => pos_start = node.pos_start,
                Nodes::WhileNode(node) => pos_start = node.pos_start,
                Nodes::CallNode(node) => pos_start = node.pos_start,
                Nodes::FunDef(node) => pos_start = node.pos_start,
                Nodes::BooleanNode(node) => pos_start = node.pos_start,
            };
        };

        let pos_end: Position;
        match node_to_call.clone() {
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

        CallNode {
            pos_start,
            pos_end,
            args: vec_args,
            node_to_call,
        }
    }
}
