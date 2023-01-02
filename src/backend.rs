use anyhow::Result;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::tc::{
    BinOpKind, CType, Constant, Expr, ExprKind, FunctionDefinition, Statement,
    TypeSpecifier,
};

const HOST_TRIPLE: Triple = Triple::host();

pub struct Backend {
    tast: FunctionDefinition,
    module: ObjectModule,
}

impl Backend {
    pub fn new(tast: FunctionDefinition) -> Result<Backend> {
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
            tast,
            module: ObjectModule::new(builder),
        })
    }

    pub fn compile(mut self) -> Result<Vec<u8>> {
        let mut context = self.module.make_context();

        let return_type = match self.tast.return_type {
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

        for statement in self.tast.body {
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
            &self.tast.name,
            Linkage::Export,
            &context.func.signature,
        )?;
        self.module.define_function(func_id, &mut context)?;

        self.module.clear_context(&mut context);

        Ok(self.module.finish().emit()?)
    }
}

fn compile_expr(function_builder: &mut FunctionBuilder, expr: Expr) -> Value {
    match expr.kind {
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
        ExprKind::Cast(inner) => {
            let from = inner.ctype;
            let to = expr.ctype;
            let value = compile_expr(function_builder, *inner);
            compile_cast(function_builder, value, from, to)
        }
        ExprKind::Constant(Constant::Int(i)) => {
            function_builder.ins().iconst(ir_type(&expr.ctype), i)
        }
        ExprKind::Constant(Constant::UnsignedInt(i)) => function_builder
            .ins()
            .iconst(ir_type(&expr.ctype), i as i64),
    }
}

fn compile_cast(
    function_builder: &mut FunctionBuilder,
    value: Value,
    from: CType,
    to: CType,
) -> Value {
    let from_ir = ir_type(&from);
    let to_ir = ir_type(&to);

    match (from_ir, to_ir) {
        (a, b)
            if a.is_int() && b.is_int() && a.lane_bits() > b.lane_bits() =>
        {
            function_builder.ins().ireduce(to_ir, value)
        }
        (b, a)
            if a.is_int() && b.is_int() && a.lane_bits() > b.lane_bits() =>
        {
            if from.is_signed() {
                function_builder.ins().sextend(to_ir, value)
            } else {
                function_builder.ins().uextend(to_ir, value)
            }
        }
        _ => unreachable!("unexpected cast from {} to {}", from, to),
    }
}

fn ir_type(ctype: &CType) -> types::Type {
    match ctype {
        CType::Int | CType::UnsignedInt => types::I32,
        CType::LongInt | CType::UnsignedLongInt => types::I64,
    }
}
