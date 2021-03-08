use crate::core::token::Token;
use crate::utils::constants::Nodes;
use crate::utils::position::Position;

#[derive(Debug, Clone)]
pub struct ForNode {
    pub var_name_token: Token,
    pub start_value: Nodes,
    pub end_value: Nodes,
    pub body_node: Nodes,
    pub step_value_node: Option<Nodes>,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl ForNode {
    pub fn new(
        var_name_token: Token,
        start_value: Nodes,
        end_value: Nodes,
        body_node: Nodes,
        step_value_node: Option<Nodes>,
    ) -> ForNode {
        let pos_start = var_name_token.clone().pos_start;
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
            Nodes::VarReassignNode(node) => pos_end = node.pos_end,
            Nodes::VarAccessNode(node) => pos_end = node.pos_end,
            Nodes::WhileNode(node) => pos_end = node.pos_end,
            Nodes::BooleanNode(node) => pos_end = node.pos_end,
            Nodes::FunDef(node) => pos_end = node.pos_end,
            Nodes::CallNode(node) => pos_end = node.pos_end,
        };

        ForNode {
            var_name_token,
            start_value,
            end_value,
            body_node,
            step_value_node,
            pos_start,
            pos_end,
        }
    }
}
