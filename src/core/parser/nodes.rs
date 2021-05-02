use crate::core::token::Token;
use crate::utils::position::Position;

// TODO(romeah): Remove pos_start and pos_end from the Node enum members
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    WhileNode {
        condition_node: Box<Node>,
        body_node: Box<Node>,
        pos_start: Position,
        pos_end: Position,
    },
    VarReassignNode {
        name: Token,
        value: Box<Node>,
        pos_start: Position,
        pos_end: Position,
    },
    VarAssignNode {
        name: Token,
        value: Box<Node>,
        reassignable: bool,
        pos_start: Position,
        pos_end: Position,
    },
    VarAccessNode {
        token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    UnaryNode {
        node: Box<Node>,
        op_token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    StringNode {
        token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    NumberNode {
        token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    IfNode {
        cases: Vec<(Node, Node)>,
        else_case: Box<Option<Node>>,
        pos_start: Position,
        pos_end: Position,
    },
    FunDef {
        name: Option<Token>,
        body_node: Box<Node>,
        arg_tokens: Vec<Token>,
        pos_start: Position,
        pos_end: Position,
    },
    ForNode {
        var_name_token: Token,
        start_value: Box<Node>,
        end_value: Box<Node>,
        body_node: Box<Node>,
        step_value_node: Box<Option<Node>>,
        pos_start: Position,
        pos_end: Position,
    },
    CharNode {
        token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    CallNode {
        node_to_call: Box<Node>,
        args: Vec<Node>,
        pos_start: Position,
        pos_end: Position,
    },
    BooleanNode {
        token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    BinOpNode {
        left: Box<Node>,
        right: Box<Node>,
        op_token: Token,
        pos_start: Position,
        pos_end: Position,
    },
    ArrayNode {
        element_nodes: Vec<Node>,
        pos_start: Position,
        pos_end: Position,
    },
    Statements {
        statements: Vec<Node>,
        pos_start: Position,
        pos_end: Position,
    },
    ReturnNode {
        value: Box<Option<Node>>,
        pos_start: Position,
        pos_end: Position,
    },
    ObjectDefNode {
        properties: Vec<(Token, Node)>,
        pos_start: Position,
        pos_end: Position,
    },
    ObjectPropAccess {
        object: Box<Node>,
        property: Token,
        pos_start: Position,
        pos_end: Position,
    },
    ObjectPropEdit {
        object: Box<Node>,
        property: Token,
        new_val: Box<Node>,
        pos_start: Position,
        pos_end: Position,
    },
    ClassDefNode {
        name: Token,
        constructor: Box<Option<Node>>,
        properties: Vec<(Token, Node)>,
        methods: Vec<(Token, Node)>,
        pos_start: Position,
        pos_end: Position,
    },
    ClassInitNode {
        name: Token,
        constructor_params: Vec<Node>,
        pos_start: Position,
        pos_end: Position,
    },
}
