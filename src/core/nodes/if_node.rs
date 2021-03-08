use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct IfNode {
    pub cases: Vec<(Nodes, Nodes)>,
    pub else_case: Option<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl IfNode {
    pub fn new(cases: Vec<(Nodes, Nodes)>, else_case: Option<Nodes>) -> IfNode {
        let first_case = &cases[0].0;
        let last_case = else_case
            .clone()
            .unwrap_or(cases[cases.len() - 1].0.clone());
        let pos_start: Position;
        match first_case.clone() {
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
            Nodes::BooleanNode(node) => pos_start = node.pos_start,
            Nodes::FunDef(node) => pos_start = node.pos_start,
            Nodes::CallNode(node) => pos_start = node.pos_start,
        };

        let pos_end: Position;
        match last_case.clone() {
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
            Nodes::BooleanNode(node) => pos_end = node.pos_end,
            Nodes::FunDef(node) => pos_end = node.pos_end,
            Nodes::CallNode(node) => pos_end = node.pos_end,
        };
        IfNode {
            cases,
            else_case,
            pos_start,
            pos_end,
        }
    }
}
