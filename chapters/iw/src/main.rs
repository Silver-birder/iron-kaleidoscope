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
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);
    let printf_fn_type = i64_type.fn_type(&[i8_ptr_type.into()], true);
    let printf_function = module.add_function("printf", printf_fn_type, None);
    
    let fn_type = i64_type.fn_type(&[i64_type.into()], false);
    let function = module.add_function("fizzbuzz", fn_type, None);
    let block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    let fb_string_ptr = builder.build_global_string_ptr("fizzbuzz", "fizzbuzz");
    let f_string_ptr = builder.build_global_string_ptr("fizz", "fizz");
    let b_string_ptr = builder.build_global_string_ptr("buzz", "buzz");
    builder.position_at_end(block);
    let x = function.get_nth_param(0).unwrap().into_int_value();
    let x3 = builder.build_int_signed_rem(x, i64_type.const_int(3, false), "rem");
    let x5 = builder.build_int_signed_rem(x, i64_type.const_int(5, false), "rem");
    let x15 = builder.build_int_signed_rem(x, i64_type.const_int(15, false), "rem");
    let cmp = builder.build_int_compare(EQ, x3, i64_type.const_int(0, false), "if");
    
    let then_bb = context.append_basic_block(function, "then");
    let else_bb = context.append_basic_block(function, "else");
    builder.build_conditional_branch(cmp, then_bb, else_bb);
    
    builder.position_at_end(then_bb);
    builder.build_call(printf_function, &[fb_string_ptr.as_pointer_value().into()], "then");

    builder.position_at_end(else_bb);
    builder.build_call(printf_function, &[f_string_ptr.as_pointer_value().into()], "then");

    // module.print_to_file("main.ll");
    let e = module.create_jit_execution_engine(OptimizationLevel::None)?;
    unsafe {
        let x = 30u64;
        let r = e.get_function::<unsafe extern "C" fn(u64)-> u64>("fizzbuzz")?.call(x);
        print!("{}", r);
    }
    Ok(())
}