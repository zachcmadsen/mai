use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::ast::{
    Constant, Expr, FunctionDefinition, Statement, TypeSpecifier,
};

const HOST_TRIPLE: Triple = Triple::host();

pub struct Backend {
    ast: FunctionDefinition,
    module: ObjectModule,
}

impl Backend {
    pub fn new(ast: FunctionDefinition) -> Backend {
        let flags_builder = settings::builder();
        let isa = isa::lookup(HOST_TRIPLE)
            .unwrap()
            .finish(settings::Flags::new(flags_builder))
            .unwrap();
        let builder = ObjectBuilder::new(
            isa,
            // TODO: Does the name matter?
            "main",
            cranelift_module::default_libcall_names(),
        )
        .unwrap();

        Backend {
            ast,
            module: ObjectModule::new(builder),
        }
    }

    pub fn compile(mut self) -> Vec<u8> {
        let mut context = self.module.make_context();

        let return_type = match self.ast.return_type {
            TypeSpecifier::Int => types::I32,
        };
        context
            .func
            .signature
            .returns
            .push(AbiParam::new(return_type));

        let mut function_builder_context = FunctionBuilderContext::new();
        let mut function_builder = FunctionBuilder::new(
            &mut context.func,
            &mut function_builder_context,
        );

        let entry_block = function_builder.create_block();
        function_builder.switch_to_block(entry_block);
        function_builder.seal_block(entry_block);

        // TODO: Is it possible to remove this clone?
        for statement in self.ast.body.0.clone() {
            match statement {
                Statement::Expr(expr) => {
                    self.compile_expr(&mut function_builder, expr);
                }
                Statement::Return(Some(expr)) => {
                    let value = self.compile_expr(&mut function_builder, expr);
                    function_builder.ins().return_(&[value]);
                }
                Statement::Return(None) => {
                    function_builder.ins().return_(&[]);
                }
                _ => unimplemented!(),
            };
        }

        function_builder.finalize();

        let func_id = self
            .module
            .declare_function(
                &self.ast.name,
                Linkage::Export,
                &context.func.signature,
            )
            .unwrap();
        self.module.define_function(func_id, &mut context).unwrap();

        self.module.clear_context(&mut context);

        self.module.finish().emit().unwrap()
    }

    fn compile_expr<'a>(
        &mut self,
        function_builder: &mut FunctionBuilder<'a>,
        expr: Expr,
    ) -> Value {
        match expr {
            Expr::Add(lhs, rhs) => {
                let lhs = self.compile_expr(function_builder, *lhs);
                let rhs = self.compile_expr(function_builder, *rhs);
                function_builder.ins().iadd(lhs, rhs)
            }
            Expr::Constant(Constant::Integer(i)) => {
                function_builder.ins().iconst(types::I32, i)
            }
            _ => unimplemented!(),
        }
    }
}
