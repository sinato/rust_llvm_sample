extern crate inkwell;

use inkwell::context::Context;
use std::{env, path};

fn main() {
    let args: Vec<_> = env::args().collect();
    let val1: u64 = args[1].parse().unwrap();
    let val2: u64 = args[2].parse().unwrap();

    // setup our builder and module
    let context = Context::create();
    let module = context.create_module("my_module");
    let builder = context.create_builder();

    // What is the difference between context.i32_type and IntType::i32_type()?
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);

    let function = module.add_function("main", fn_type, None);
    let basic_block = context.append_basic_block(&function, "entry");

    builder.position_at_end(&basic_block);

    let a = i32_type.const_int(val1, false);

    let b = i32_type.const_int(val2, false);
    /*
    let a_ref = builder.build_alloca(i32_type, "a");
    builder.build_store(a_ref, a);
    let b_ref = builder.build_alloca(i32_type, "b");
    builder.build_store(b_ref, b);

    builder.build_load(b_ref, "b_val");
    builder.build_load(a_ref, "a_val");
    */
    let res = builder.build_int_add(a, b, "ab_val");

    builder.build_return(Some(&res));

    let _ = module.print_to_file(path::Path::new("compiled.ll"));
}
