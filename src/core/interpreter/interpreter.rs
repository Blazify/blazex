use crate::{
    core::{
        interpreter::{runtime_result::RuntimeResult, value::Value},
        parser::nodes::Node,
        token::Token,
    },
    utils::{
        constants::{DynType, Tokens},
        context::Context,
        error::Error,
        symbol::Symbol,
    },
    LanguageServer,
};

use crate::std::lib::init_std;
use std::collections::HashMap;

// TODO: Remove Interpreter after VM is completed
pub struct Interpreter {
    pub ctx: Vec<Context>,
}

impl LanguageServer for Interpreter {
    type Result = Result<Value, Error>;
    fn from_ast(name: &'static str, node: Node) -> Self::Result {
        let mut ctx = Context::new("Global".to_string());
        init_std(&mut ctx);
        let ctx_ = Context::new(name.to_string());
        let vec_ctx = vec![ctx.clone(), ctx_];

        let res = Self::new(vec_ctx).interpret_node(node.clone());
        return if res.val.is_some() {
            return if res.should_return() {
                Ok(res.val.unwrap())
            } else {
                Ok(Value::Null)
            };
        } else {
            Err(res.error.unwrap())
        };
    }
}

impl Interpreter {
    pub fn new(ctx: Vec<Context>) -> Interpreter {
        Self { ctx }
    }

    pub fn interpret_node(&mut self, node: Node) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        match node {
            Node::NumberNode {
                token,
                pos_start,
                pos_end,
            } => {
                if let DynType::Int(_) = token.value {
                    res.success(Value::Int {
                        val: token.value.into_int(),
                        pos_start,
                        pos_end,
                    })
                } else {
                    res.success(Value::Float {
                        val: token.value.into_float(),
                        pos_start,
                        pos_end,
                    })
                }
            }
            Node::StringNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Value::String {
                val: token.value.into_string(),
                pos_start,
                pos_end,
            }),
            Node::CharNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Value::Char {
                val: token.value.into_char(),
                pos_start,
                pos_end,
            }),
            Node::BinOpNode {
                left,
                right,
                op_token,
                ..
            } => {
                let left_built = self.interpret_node(*left);
                if left_built.should_return() {
                    return left_built;
                }

                let right_built = self.interpret_node(*right);
                if right_built.should_return() {
                    return right_built;
                }
                left_built
                    .val
                    .unwrap()
                    .op(right_built.val.unwrap(), op_token)
            }
            Node::UnaryNode { node, op_token, .. } => {
                let built = self.interpret_node(*node);
                if built.should_return() {
                    return built;
                }

                built.val.unwrap().unary(op_token.r#type)
            }
            Node::BooleanNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Value::Boolean {
                val: token.value.into_boolean(),
                pos_start,
                pos_end,
            }),
            Node::VarAssignNode {
                name,
                value,
                reassignable,
                pos_start,
                pos_end,
            } => {
                let val = self.interpret_node(*value);
                if val.should_return() {
                    return val;
                }

                if self
                    .get_from_ctx(name.clone().value.into_string())
                    .is_some()
                {
                    return res.failure(
                        Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Variable Declared Twice",
                        )
                        .set_ctx(self.ctx.clone()),
                    );
                }

                self.ctx.last_mut().unwrap().symbols.insert(
                    name.value.into_string(),
                    Symbol::new(&val.val.unwrap(), reassignable),
                );
                res.success(Value::Null)
            }
            Node::VarReassignNode {
                value,
                name,
                pos_start,
                pos_end,
            } => {
                let result = self.get_from_ctx(name.clone().value.into_string());
                if result.clone().is_none() {
                    return res.failure(
                        Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Variable not found to be reassigned",
                        )
                        .set_ctx(self.ctx.clone()),
                    );
                }

                if result.clone().unwrap().reassignable == false {
                    return res.failure(
                        Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Variable not reassignable",
                        )
                        .set_ctx(self.ctx.clone()),
                    );
                }

                let val = self.interpret_node(*value);
                if val.should_return() {
                    return val;
                }

                self.get_and_set_ctx(
                    name.value.into_string(),
                    Symbol::new(&val.val.unwrap(), true),
                );

                res.success(Value::Null)
            }
            Node::VarAccessNode {
                token,
                pos_start,
                pos_end,
            } => {
                let result = self.get_from_ctx(token.clone().value.into_string());

                if result.clone().is_none() {
                    return res.failure(
                        Error::new("Runtime Error", pos_start, pos_end, "Variable not found")
                            .set_ctx(self.ctx.clone()),
                    );
                }

                res.success(result.unwrap().value)
            }
            Node::IfNode {
                cases, else_case, ..
            } => {
                for (condition, expression) in cases {
                    let condition_val = self.interpret_node(condition);
                    if condition_val.should_return() {
                        return condition_val;
                    }

                    if condition_val.clone().val.unwrap().is_true() {
                        let expr_val = self.interpret_node(expression);
                        if expr_val.should_return() {
                            return expr_val;
                        }
                    }
                }
                if else_case.is_some() {
                    let else_val = self.interpret_node(else_case.unwrap());
                    if else_val.should_return() {
                        return else_val;
                    }
                }
                res.success(Value::Null)
            }
            Node::ForNode {
                var_name_token,
                start_value,
                body_node,
                step_value_node,
                end_value,
                pos_start,
                pos_end,
            } => {
                let start = self.interpret_node(*start_value.clone());
                if start.should_return() {
                    return start;
                }

                let end = self.interpret_node(*end_value.clone());
                if end.should_return() {
                    return end;
                }

                let step_value;
                if step_value_node.is_some() {
                    step_value = self
                        .interpret_node(step_value_node.unwrap().clone())
                        .val
                        .unwrap_or(Value::Int {
                            val: 1,
                            pos_start,
                            pos_end,
                        });
                } else {
                    step_value = Value::Int {
                        val: 1,
                        pos_start,
                        pos_end,
                    };
                }

                let mut i = start.val.unwrap().get_int();

                let condition;
                if step_value.clone().get_int() >= 0 {
                    condition = i < end.clone().val.unwrap().get_int();
                } else {
                    condition = i > end.clone().val.unwrap().get_int();
                }

                while condition {
                    if i == end.clone().val.unwrap().get_int() {
                        break;
                    }
                    self.ctx.last_mut().unwrap().symbols.insert(
                        var_name_token.clone().value.into_string(),
                        Symbol::new(
                            &Value::Int {
                                val: i,
                                pos_start,
                                pos_end,
                            },
                            true,
                        ),
                    );

                    i += step_value.clone().get_int();
                    let body_eval = self.interpret_node(*body_node.clone());
                    if body_eval.should_return() {
                        return body_eval;
                    }
                }

                res.success(Value::Null)
            }
            Node::WhileNode {
                condition_node,
                body_node,
                ..
            } => {
                loop {
                    let condition = self.interpret_node(*condition_node.clone());
                    if condition.should_return() {
                        return condition;
                    }

                    if !condition.clone().val.unwrap().is_true() {
                        break;
                    }

                    let body_eval = self.interpret_node(*body_node.clone());
                    if body_eval.should_return() {
                        return body_eval;
                    }
                }

                res.success(Value::Null)
            }
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
                pos_start,
                pos_end,
            } => {
                let mut func_name = name.unwrap_or(Token::new(
                    Tokens::Identifier,
                    pos_start,
                    pos_end,
                    DynType::String("anonymous".to_string()),
                ));

                let mut object = None;
                if self.ctx.last().unwrap().display_name.starts_with("Class") {
                    if func_name.clone().value.into_string() == "anonymous".to_string() {
                        func_name = Token::new(
                            Tokens::Identifier,
                            pos_start,
                            pos_end,
                            DynType::String("constructor".to_string()),
                        );
                    }

                    object = Some((*self.ctx.last().unwrap().symbols.get("soul").unwrap()).clone());
                }

                let val = Value::Function {
                    name: func_name.clone(),
                    body_node: *body_node.clone(),
                    args: arg_tokens,
                    pos_start,
                    pos_end,
                    object: Box::new(object),
                };

                let v_cl = val.clone();

                if !["anonymous", "constructor"]
                    .contains(&&func_name.clone().value.into_string().as_str())
                {
                    self.ctx.last_mut().unwrap().symbols.insert(
                        func_name.clone().value.into_string(),
                        Symbol::new(&val, true),
                    );
                }

                res.success(v_cl)
            }
            Node::CallNode {
                node_to_call, args, ..
            } => {
                let mut eval_args: Vec<Value> = vec![];
                let val_to_call = self.interpret_node(*node_to_call.clone());

                if val_to_call.error.is_some() {
                    return val_to_call;
                }

                for arg in args {
                    let eval_arg = self.interpret_node(arg);
                    if eval_arg.error.is_some() {
                        return eval_arg;
                    }

                    eval_args.push(eval_arg.val.unwrap());
                }

                let val = val_to_call.val.unwrap().execute(eval_args, self);
                if val.error.is_some() {
                    return val;
                }
                res.success(val.clone().val.unwrap())
            }
            Node::ArrayNode {
                element_nodes,
                pos_start,
                pos_end,
            } => {
                let mut elements: Vec<Value> = vec![];

                for node in element_nodes {
                    let element = self.interpret_node(node);
                    if element.should_return() {
                        return element;
                    }
                    elements.push(element.val.unwrap());
                }

                res.success(Value::Array {
                    elements,
                    pos_start,
                    pos_end,
                })
            }
            Node::Statements {
                statements,
                pos_start,
                pos_end,
            } => {
                let mut elements: Vec<Value> = vec![];

                for node in statements {
                    let element = self.interpret_node(node);
                    if element.should_return() {
                        return element;
                    }
                    elements.push(element.val.unwrap());
                }

                res.success(Value::Array {
                    elements,
                    pos_start,
                    pos_end,
                })
            }
            Node::ReturnNode { value, .. } => {
                let val;
                if value.is_some() {
                    let eval_val = self.interpret_node(value.unwrap().clone());
                    if eval_val.should_return() {
                        return eval_val;
                    }
                    val = eval_val.val.unwrap();
                } else {
                    val = Value::Null;
                }

                res.success_return(val)
            }
            Node::ObjectDefNode {
                properties,
                pos_start,
                pos_end,
            } => {
                let mut hash_props = HashMap::new();
                for (k, v) in properties {
                    let e_v = self.interpret_node(v);
                    if e_v.should_return() {
                        return e_v;
                    }

                    if hash_props.contains_key(&k.clone().value.into_string()) {
                        return res.failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Key already exits",
                        ));
                    }
                    hash_props.insert(k.value.into_string(), e_v.val.unwrap());
                }

                res.success(Value::Object {
                    properties: hash_props,
                    pos_start,
                    pos_end,
                })
            }
            Node::ObjectPropAccess {
                object, property, ..
            } => {
                let obj = self.interpret_node(*object.clone());
                if obj.should_return() {
                    return obj;
                }

                let prop = obj
                    .val
                    .unwrap()
                    .get_obj_prop_val(property.value.into_string());
                if prop.should_return() {
                    return prop;
                }

                res.success(prop.val.unwrap())
            }
            Node::ObjectPropEdit {
                new_val,
                property,
                object,
                ..
            } => {
                let obj = self.interpret_node(*object.clone());
                if obj.should_return() {
                    return obj;
                }

                let new_val_e = self.interpret_node(*new_val.clone());
                if new_val_e.should_return() {
                    return new_val_e;
                }

                let prop = obj
                    .val
                    .unwrap()
                    .set_obj_prop_val(property.value.into_string(), new_val_e.val.unwrap());
                if prop.should_return() {
                    return prop;
                }

                res.success(prop.val.unwrap())
            }
            Node::ClassDefNode {
                name,
                constructor,
                methods,
                properties,
                pos_start,
                pos_end,
            } => {
                let mut methods_e: HashMap<String, Node> = HashMap::new();
                let mut properties_e: HashMap<String, Node> = HashMap::new();

                for (k, v) in properties {
                    if properties_e.contains_key(&k.clone().value.into_string()) {
                        return res.failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Key already exits",
                        ));
                    }
                    properties_e.insert(k.value.into_string(), v);
                }
                for (k, v) in methods {
                    if methods_e.contains_key(&k.clone().value.into_string()) {
                        return res.failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Key already exits",
                        ));
                    }
                    methods_e.insert(k.value.into_string(), v);
                }

                let class = Value::Class {
                    pos_start,
                    pos_end,
                    name: name.clone().value.into_string(),
                    constructor: *constructor.clone(),
                    methods: methods_e,
                    properties: properties_e,
                };

                self.ctx
                    .last_mut()
                    .unwrap()
                    .symbols
                    .insert(name.clone().value.into_string(), Symbol::new(&class, false));

                res.success(class)
            }
            Node::ClassInitNode {
                name,
                constructor_params,
                pos_start,
                pos_end,
            } => {
                let mut constructor_params_e: Vec<Value> = vec![];
                let class = self.get_from_ctx(name.value.into_string());
                if class.clone().is_none() {
                    return res.failure(
                        Error::new("Runtime Error", pos_start, pos_end, "Unknown Class")
                            .set_ctx(self.ctx.clone()),
                    );
                }

                for param in constructor_params {
                    let param_e = self.interpret_node(param);
                    if param_e.should_return() {
                        return param_e;
                    }

                    constructor_params_e.push(param_e.val.unwrap());
                }

                class.unwrap().value.init_class(constructor_params_e, self)
            }
        }
    }

    pub fn get_from_ctx(&self, k: String) -> Option<Symbol> {
        for idx in (0..self.ctx.len()).rev() {
            let sym = self.ctx.get(idx).unwrap().symbols.get(&k);
            if sym.is_some() {
                return Some((*sym.unwrap()).clone());
            }
        }
        None
    }

    pub fn get_and_set_ctx(&mut self, k: String, n: Symbol) {
        for idx in (0..self.ctx.len()).rev() {
            let sym = self.ctx.get(idx).unwrap().symbols.get(&k.clone());
            if sym.is_some() {
                self.ctx.get_mut(idx).unwrap().symbols.insert(k, n);
                break;
            }
        }
    }
}
