use anyhow::Result;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::ast::{
    BinOpKind, Constant, ExprKind, FunctionDefinition, Statement,
    TypeSpecifier,
};

const HOST_TRIPLE: Triple = Triple::host();

pub struct Backend {
    ast: FunctionDefinition,
    module: ObjectModule,
}

impl Backend {
    pub fn new(ast: FunctionDefinition) -> Result<Backend> {
        let flags_builder = settings::builder();
        let isa = isa::lookup(HOST_TRIPLE)?
            .finish(settings::Flags::new(flags_builder))?;
        let builder = ObjectBuilder::new(
            isa,
            // TODO: Does the name matter?
            "main",
            cranelift_module::default_libcall_names(),
        )?;

        Ok(Backend {
            ast,
            module: ObjectModule::new(builder),
        })
    }

    pub fn compile(mut self) -> Result<Vec<u8>> {
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

        for statement in self.ast.body {
            match statement {
                Statement::Expr(expr) => {
                    compile_expr(&mut function_builder, expr);
                }
                Statement::Return(Some(expr)) => {
                    let value = compile_expr(&mut function_builder, expr);
                    function_builder.ins().return_(&[value]);
                }
                Statement::Return(None) => {
                    function_builder.ins().return_(&[]);
                }
                _ => unimplemented!(),
            };
        }

        function_builder.finalize();

        let func_id = self.module.declare_function(
            &self.ast.name,
            Linkage::Export,
            &context.func.signature,
        )?;
        self.module.define_function(func_id, &mut context)?;

        self.module.clear_context(&mut context);

        Ok(self.module.finish().emit()?)
    }
}

fn compile_expr(
    function_builder: &mut FunctionBuilder,
    expr: ExprKind,
) -> Value {
    match expr {
        ExprKind::Binary(BinOpKind::Add, lhs, rhs) => {
            let lhs = compile_expr(function_builder, *lhs);
            let rhs = compile_expr(function_builder, *rhs);
            function_builder.ins().iadd(lhs, rhs)
        }
        ExprKind::Binary(BinOpKind::Sub, lhs, rhs) => {
            let lhs = compile_expr(function_builder, *lhs);
            let rhs = compile_expr(function_builder, *rhs);
            function_builder.ins().isub(lhs, rhs)
        }
        ExprKind::Constant(Constant::Integer(i)) => {
            function_builder.ins().iconst(types::I32, i)
        }
    }
}
