extern crate llvm_sys;

use llvm_sys::core::*;
use llvm_sys::analysis::{LLVMVerifyModule, LLVMVerifierFailureAction};
use llvm_sys::execution_engine::LLVMCreateExecutionEngineForModule;
use std::ffi::CString;
use std::os::raw::{c_char};

fn main() {
    // setup our builder and module
    //builder := llvm.NewBuilder()
    //mod := llvm.NewModule("my_module")
    let builder = unsafe { LLVMCreateBuilder() };
    let mod_name = CString::new("my_module").unwrap();
    let module = unsafe { LLVMModuleCreateWithName(mod_name.as_ptr()) };


    // create our function prologue
    //main := llvm.FunctionType(llvm.Int32Type(), []llvm.Type{}, false)
    //llvm.AddFunction(mod, "main", main)
    //block := llvm.AddBasicBlock(mod.NamedFunction("main"), "entry")
    //builder.SetInsertPoint(block, block.FirstInstruction())
    let function_type = unsafe {
        let mut param_types = [];
        LLVMFunctionType(LLVMInt32Type(), param_types.as_mut_ptr(), param_types.len() as u32, 0)
    };
    let function_name = CString::new("main").unwrap();
    let function = unsafe { LLVMAddFunction(module, function_name.as_ptr(), function_type) };
//    let named_function = unsafe { LLVMGetNamedFunction(module, function_name.as_ptr()) };
    let entry_name = CString::new("entry").unwrap();
    let entry_block = unsafe { LLVMAppendBasicBlock(function, entry_name.as_ptr()) };
    unsafe { LLVMPositionBuilderAtEnd(builder, entry_block); }

    // int a = 32
    //a := builder.CreateAlloca(llvm.Int32Type(), "a")
    //builder.CreateStore(llvm.ConstInt(llvm.Int32Type(), 32, false), a)
    let a_name = CString::new("a").unwrap();
    let a = unsafe { LLVMBuildAlloca(builder, LLVMInt32Type(), a_name.as_ptr()) };
    unsafe { LLVMBuildStore(builder, LLVMConstInt(LLVMInt32Type(), 32, 0), a); }

    // int b = 16
    //b := builder.CreateAlloca(llvm.Int32Type(), "b")
    //builder.CreateStore(llvm.ConstInt(llvm.Int32Type(), 16, false), b)
    let b_name = CString::new("b").unwrap();
    let b = unsafe { LLVMBuildAlloca(builder, LLVMInt32Type(), b_name.as_ptr()) };
    unsafe { LLVMBuildStore(builder, LLVMConstInt(LLVMInt32Type(), 16, 0), b); }

    // return a + b
    //bVal := builder.CreateLoad(b, "b_val")
    //aVal := builder.CreateLoad(a, "a_val")
    //result := builder.CreateAdd(aVal, bVal, "ab_val")
    //builder.CreateRet(result)
    let b_val_name = CString::new("b_val").unwrap();
    let b_val = unsafe { LLVMBuildLoad(builder, b, b_val_name.as_ptr()) };
    let a_val_name = CString::new("a_val").unwrap();
    let a_val = unsafe { LLVMBuildLoad(builder, a, a_val_name.as_ptr()) };

    let ab_val_name = CString::new("ab_val").unwrap();
    unsafe {
        let res = LLVMBuildAdd(builder, a_val, b_val, ab_val_name.as_ptr());
        LLVMBuildRet(builder, res);
    }

    // verify it's all good
    //if ok := llvm.VerifyModule(mod, llvm.ReturnStatusAction); ok != nil {
    //    fmt.Println(ok.Error())
    //}
    let mut error: *mut c_char = 0 as *mut c_char;
    let ok = unsafe {
        let buf: *mut *mut c_char = &mut error;
        LLVMVerifyModule(module, LLVMVerifierFailureAction::LLVMReturnStatusAction, buf)
    };
    if ok != 0 {
        let err_msg = unsafe { CString::from_raw(error).into_string().unwrap() };
        panic!("cannot verify module '{:?}'.\nError: {}", mod_name, err_msg);
    }

    // Clean up the builder now that we are finished using it.
    unsafe { LLVMDisposeBuilder(builder) }

    //mod.Dump()
    // Dump the LLVM IR to stdout so we can see what we've created
    unsafe { LLVMDumpModule(module) }


/* 
    // create our exe engine
    engine, err := llvm.NewExecutionEngine(mod)
    if err != nil {
        fmt.Println(err.Error())
    }
 
    // run the function!
    funcResult := engine.RunFunction(mod.NamedFunction("main"), []llvm.GenericValue{})
    fmt.Printf("%d\n", funcResult.Int(false))
*/

    // Clean up the module after we're done with it.
    unsafe { LLVMDisposeModule(module) }
}
