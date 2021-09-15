extern crate inkwell;

use inkwell::context::Context;
use inkwell::OptimizationLevel;

fn main() {
    let context = Context::create();
    // moduleを作成
    let module = context.create_module("main");
    // builderを作成
    let builder = context.create_builder();

    // // 型関係の変数
    let i32_type = context.i32_type();
    // let i32_type = context.i32_type();
    // let i8_type = context.i8_type();
    // let i8_ptr_type = i8_type.ptr_type(inkwell::AddressSpace::Generic);

    let sum_fn_type = i32_type.fn_type(&[i32_type.into(), i32_type.into()], false);
    let sum_function = module.add_function("sum", sum_fn_type, None);
    // let basic_block = context.append_basic_block(sum_function, "entry");

    let x = sum_function.get_nth_param(0).unwrap().into_int_value();
    let y = sum_function.get_nth_param(1).unwrap().into_int_value();
    let sum = builder.build_int_add(x, y, "sum");
    builder.build_return(Some(&sum));

    // // printf関数を宣言
    // let printf_fn_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
    // let printf_function = module.add_function("printf", printf_fn_type, None);

    // main関数を宣言
    let main_fn_type = i32_type.fn_type(&[], false);
    let main_function = module.add_function("main", main_fn_type, None);

    // main関数にBasic Blockを追加
    let entry_basic_block = context.append_basic_block(main_function, "entry");
    // builderのpositionをentry Basic Blockに設定
    builder.position_at_end(entry_basic_block);

    builder.build_call(sum_function, &[], "call");

    // // ここからmain関数に命令をビルドしていく
    // // globalに文字列を宣言
    // let hw_string_ptr = builder.build_global_string_ptr("Hello, world!", "hw");
    // // printfをcall
    // builder.build_call(printf_function, &[hw_string_ptr.as_pointer_value().into()], "call");
    // // main関数は0を返す
    // builder.build_return(Some(&i32_type.const_int(0, false)));

    // JIT実行エンジンを作成し、main関数を実行
    let execution_engine = module.create_jit_execution_engine(OptimizationLevel::Aggressive).unwrap();
    unsafe {
        execution_engine.get_function::<unsafe extern "C" fn()>("main").unwrap().call();
    }
}