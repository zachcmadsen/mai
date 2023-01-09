use anyhow::Result;
use cranelift::prelude::*;
use cranelift_module::{Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;

use crate::tast::{Expr, ExprKind};

pub const HOST_TRIPLE: Triple = Triple::host();

pub fn gen(expr: &Expr) -> Result<Vec<u8>> {
    let flags_builder = settings::builder();
    let isa = isa::lookup(HOST_TRIPLE)?
        .finish(settings::Flags::new(flags_builder))?;
    let builder = ObjectBuilder::new(
        isa,
        // TODO: Does the name matter?
        "main",
        cranelift_module::default_libcall_names(),
    )?;
    let mut module = ObjectModule::new(builder);

    let mut context = module.make_context();
    context
        .func
        .signature
        .returns
        .push(AbiParam::new(types::I32));
    let mut builder_context = FunctionBuilderContext::new();
    let mut builder =
        FunctionBuilder::new(&mut context.func, &mut builder_context);

    let entry_block = builder.create_block();
    builder.switch_to_block(entry_block);
    builder.seal_block(entry_block);

    let return_value = gen_expr(expr, &mut builder);
    builder.ins().return_(&[return_value]);

    let func_id = module.declare_function(
        "main",
        Linkage::Export,
        &context.func.signature,
    )?;
    module.define_function(func_id, &mut context)?;

    module.clear_context(&mut context);

    Ok(module.finish().emit()?)
}

fn gen_expr(expr: &Expr, builder: &mut FunctionBuilder) -> Value {
    match expr.kind {
        ExprKind::Add(lhs, rhs) => {
            let lhs = gen_expr(lhs, builder);
            let rhs = gen_expr(rhs, builder);
            builder.ins().iadd(lhs, rhs)
        }
        ExprKind::Int(int) => builder.ins().iconst(types::I32, int as i64),
    }
}
