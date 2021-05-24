use bzs_shared::{Node, Tokens};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    values::{FloatValue, FunctionValue},
};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn compile_node(&mut self, node: Node) -> Result<FloatValue<'ctx>, &'static str> {
        match node {
            Node::Statements { statements } => {
                for statement in statements {
                    // TODO: Implement multiple statements
                    return self.compile_node(statement);
                }
                Err("")
            }
            Node::NumberNode { token } => Ok(self
                .context
                .f64_type()
                .const_float(token.value.into_float())),
            Node::BinaryNode {
                left,
                op_token,
                right,
            } => {
                let lhs = self.compile_node(*left)?;
                let rhs = self.compile_node(*right)?;

                match op_token.typee {
                    Tokens::Plus => Ok(self.builder.build_float_add(lhs, rhs, "tmpadd")),
                    Tokens::Minus => Ok(self.builder.build_float_sub(lhs, rhs, "tmpsub")),
                    Tokens::Multiply => Ok(self.builder.build_float_mul(lhs, rhs, "tmpmul")),
                    Tokens::Divide => Ok(self.builder.build_float_div(lhs, rhs, "tmpdiv")),
                    _ => Err("Unknown op"),
                }
            }
            _ => Err("Not implemented please don't use -l argument for this program"),
        }
    }

    fn compile_top(&mut self, node: Node) -> Result<FunctionValue<'ctx>, &'static str> {
        let f_64 = self.context.f64_type();
        let fn_type = f_64.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        self.builder.build_return(Some(&self.compile_node(node)?));

        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            unsafe {
                function.delete();
            }

            Err("Invalid generated function.")
        }
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        node: Node,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
        let mut compiler = Compiler {
            builder,
            context,
            module,
            fpm,
        };

        compiler.compile_top(node)
    }
}

pub fn init_compiler(node: Node) {
    let context = Context::create();
    let module = context.create_module("repl");
    let builder = context.create_builder();

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    match Compiler::compile(&context, &builder, &module, &fpm, node) {
        Ok(function) => {
            println!("Expression compiled to IR:");
            function.print_to_stderr();
        }
        Err(err) => {
            println!("Error compiling function: {}", err);
        }
    }
}
