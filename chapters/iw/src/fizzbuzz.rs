extern crate inkwell;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module;
use inkwell::IntPredicate::EQ;
use inkwell::OptimizationLevel;
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
    let void_type = context.void_type();
    let i8_type = context.i8_type();
    let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);
    let null = i8_ptr_type.const_null();
    let printf_fn_type = void_type.fn_type(&[i8_ptr_type.into()], true);
    let printf_function = module.add_function("printf", printf_fn_type, None);

    let fn_type = i64_type.fn_type(&[i64_type.into()], false);
    let function = module.add_function("fizzbuzz", fn_type, None);
    let block = context.append_basic_block(function, "entry");
    let builder = context.create_builder();
    builder.position_at_end(block);
    let fb_string_ptr = builder.build_global_string_ptr("fizzbuzz\n", "fizzbuzz");
    let f_string_ptr = builder.build_global_string_ptr("fizz\n", "fizz");
    let b_string_ptr = builder.build_global_string_ptr("buzz\n", "buzz");
    let x = function.get_nth_param(0).unwrap().into_int_value();
    let x3 = builder.build_int_signed_rem(x, i64_type.const_int(3, false), "rem");
    let x5 = builder.build_int_signed_rem(x, i64_type.const_int(5, false), "rem");
    let x15 = builder.build_int_signed_rem(x, i64_type.const_int(15, false), "rem");
    let x3_cmp = builder.build_int_compare(EQ, x3, i64_type.const_int(0, false), "if");
    let x5_cmp = builder.build_int_compare(EQ, x5, i64_type.const_int(0, false), "if");
    let x15_cmp = builder.build_int_compare(EQ, x15, i64_type.const_int(0, false), "if");
    let fb_then_bb = context.append_basic_block(function, "fb_then");
    let con_1_bb = context.append_basic_block(function, "con_1");
    let con_2_bb = context.append_basic_block(function, "con_2");
    let f_else_bb = context.append_basic_block(function, "f_else_if");
    let b_else_bb = context.append_basic_block(function, "b_else");
    let cont_bb = context.append_basic_block(function, "ifcont");
    builder.build_conditional_branch(x15_cmp, fb_then_bb, con_1_bb);

    builder.position_at_end(fb_then_bb);
    builder.build_call(
        printf_function,
        &[fb_string_ptr.as_pointer_value().into()],
        "c_fb",
    );
    builder.build_unconditional_branch(cont_bb);

    builder.position_at_end(con_1_bb);
    builder.build_conditional_branch(x3_cmp, f_else_bb, con_2_bb);

    builder.position_at_end(f_else_bb);
    builder.build_call(
        printf_function,
        &[f_string_ptr.as_pointer_value().into()],
        "c_f",
    );
    builder.build_unconditional_branch(cont_bb);

    builder.position_at_end(con_2_bb);
    builder.build_conditional_branch(x5_cmp, b_else_bb, cont_bb);

    builder.position_at_end(b_else_bb);
    builder.build_call(
        printf_function,
        &[b_string_ptr.as_pointer_value().into()],
        "c_b",
    );
    builder.build_unconditional_branch(cont_bb);

    builder.position_at_end(cont_bb);
    builder.build_return(Some(&null));

    // module.print_to_file("main.ll");
    let e = module.create_jit_execution_engine(OptimizationLevel::None)?;
    unsafe {
        let x = 6u64;
        e.get_function::<unsafe extern "C" fn(u64) -> ()>("fizzbuzz")?
            .call(x);
    }
    Ok(())
}
