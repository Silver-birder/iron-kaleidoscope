extern crate inkwell;

use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::{ExecutionEngine};
use inkwell::module::Module;
use std::error::Error;

struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    execution_engine: ExecutionEngine<'ctx>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::create();
    let module = context.create_module("main");
    let builder = context.create_builder();
    let i64_type = context.i64_type();
    let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
    let function = module.add_function("sum", fn_type, None);
    let basic_block = context.append_basic_block(function, "entry");
    builder.position_at_end(basic_block); // module > function > block を定義させてから、buildを動かすんだ!

    let x = function.get_nth_param(0).unwrap().into_int_value();
    let y = function.get_nth_param(1).unwrap().into_int_value();
    let z = function.get_nth_param(2).unwrap().into_int_value();
    let sum = builder.build_int_add(x, y, "sum");
    let sum = builder.build_int_add(z, sum, "sum");
    builder.build_return(Some(&sum));

    let e = module.create_jit_execution_engine(OptimizationLevel::None)?;

    // 途中経過は、module.print_to_fileでllファイルを読むんだ! (RUST_BACKTRACE=1も大切だ)

    unsafe { 
        let x = 1u64;
        let y = 2u64;
        let z = 3u64;
        let s = e.get_function::<unsafe extern "C" fn(u64, u64, u64)-> u64>("sum")?.call(x, y , z);
        print!("{}", s);
    };
    Ok(())
}