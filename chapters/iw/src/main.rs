extern crate inkwell;

use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine};
use inkwell::module::Module;
use inkwell::IntPredicate::EQ;
use std::error::Error;
use std::ptr::null;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("fizzbuzz");
    let i64_type = context.i64_type();
    let f64_type = context.f64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into()], false);
    let function = module.add_function("fizzbuzz", fn_type, None);
    let block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(block);
    let x = function.get_nth_param(0).unwrap().into_int_value();
    let x3 = builder.build_int_signed_rem(x, i64_type.const_int(3, false), "rem");
    let x5 = builder.build_int_signed_rem(x, i64_type.const_int(5, false), "rem");
    let x15 = builder.build_int_signed_rem(x, i64_type.const_int(15, false), "rem");
    let cmp = builder.build_int_compare(EQ, x3, i64_type.const_int(0, false), "EQ");
    builder.build_unsigned_int_to_float(cmp, f64_type, "cmp");
    builder.build_return(Some(&x15));

    let e = module.create_jit_execution_engine(OptimizationLevel::None)?;
    unsafe {
        let x = 3u64;
        let r = e.get_function::<unsafe extern "C" fn(u64)-> u64>("fizzbuzz")?.call(x);
        print!("{}", r);
    }
    Ok(())
}