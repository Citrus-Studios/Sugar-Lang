use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::path::Path;
use std::process::exit;
use std::process::Command;

use llvm_sys::analysis::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;

use crate::parser::{Expr, Expr_};

pub unsafe fn compile_llvm(ast: Vec<Expr>) {
    let module = LLVMModuleCreateWithName(b"main\0".as_ptr() as *const _);

    let mut functions = HashMap::new();

    for x in ast {
        match x.node {
            Expr_::Declare(name, types) => {
                let types = types
                    .into_iter()
                    .map(|t| match t.as_str() {
                        "byte" => LLVMInt8Type(),
                        "void" => LLVMVoidType(),
                        _ => panic!("Unknown Type"),
                    })
                    .collect::<Vec<LLVMTypeRef>>();
                let return_type = types[types.len() - 1];
                let args = &types[..types.len() - 2];
                let function_sig =
                    LLVMFunctionType(return_type, args.as_ptr() as *mut _, args.len() as u32, 0);

                let name_c = CString::new(name.clone()).unwrap();
                let func = LLVMAddFunction(module, name_c.as_ptr(), function_sig);
                functions.insert(name, func);
            }
            Expr_::Define(name, _args, expr) => {
                let func = *functions.get_key_value(&name).unwrap().1;
                let entry_name = CString::new("entry").unwrap();
                let entry = LLVMAppendBasicBlock(func, entry_name.as_ptr());
                let builder = LLVMCreateBuilder();
                LLVMPositionBuilderAtEnd(builder, entry);
                let mut variables = HashMap::new();
                if expr.len() == 1 {
                    match expr[0].node {
                        Expr_::Byte(v) => {
                            LLVMBuildRet(builder, LLVMConstInt(LLVMInt8Type(), v.into(), 0));
                        }
                        _ => todo!(),
                    }
                } else {
                    for y in expr {
                        match y.node {
                            Expr_::Assign(name, expr) => {
                                let name = name.clone();
                                let alloced = LLVMBuildAlloca(
                                    builder,
                                    LLVMInt8Type(),
                                    name.as_ptr() as *mut _,
                                );
                                LLVMBuildStore(
                                    builder,
                                    match_expr(expr.node.clone(), &mut variables, builder),
                                    alloced,
                                );
                                variables.insert(name.clone(), alloced);
                            }
                            Expr_::Byte(v) => {
                                LLVMBuildRet(builder, LLVMConstInt(LLVMInt8Type(), v.into(), 0));
                            }
                            Expr_::Var(v) => {
                                LLVMBuildRet(
                                    builder,
                                    LLVMBuildLoad2(
                                        builder,
                                        LLVMInt8Type(),
                                        *variables.get_key_value(&v).unwrap().1,
                                        v.as_ptr() as *const _,
                                    ),
                                );
                            }
                            _ => todo!(),
                        }
                    }
                }
            }
            _ => todo!(),
        }
    }
    let mut error = std::ptr::null_mut() as *mut i8;

    LLVMVerifyModule(
        module,
        LLVMVerifierFailureAction::LLVMAbortProcessAction,
        &mut error,
    );
    LLVMDisposeMessage(error);

    let mut engine = std::ptr::null_mut();
    let mut error = std::ptr::null_mut() as *mut i8;

    LLVMLinkInMCJIT();
    LLVM_InitializeNativeTarget();

    if LLVMCreateExecutionEngineForModule(&mut engine, module, &mut error) == 1 {
        eprintln!("failed to create execution engine");
        exit(1);
    }

    if error != std::ptr::null_mut() {
        eprintln!("error: {:?}", error);
        LLVMDisposeMessage(error);
        exit(1);
    }

    let builddir = Path::new("./build");
    env::set_current_dir(&builddir).unwrap();

    if LLVMWriteBitcodeToFile(module, b"out.bc\0".as_ptr() as *const _) != 0 {
        eprintln!("error writing bitcode to file, skipping");
    }

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "llvm-dis out.bc"])
            .output()
            .unwrap();

        Command::new("cmd")
            .args(["/C", "llc -filetype=obj out.ll -o out.o"])
            .output()
            .unwrap();

        Command::new("cmd")
            .args(["/C", "clang out.o -o out;"])
            .output()
            .unwrap();
    } else {
        Command::new("sh")
            .args(["-c", "llvm-dis out.bc"])
            .output()
            .unwrap();

        Command::new("sh")
            .args(["-c", "llc -filetype=obj out.ll -o out.o"])
            .output()
            .unwrap();

        Command::new("sh")
            .args(["-c", "clang out.o -o out;"])
            .output()
            .unwrap();
    };
}

unsafe fn match_expr(
    expr: Expr_,
    variables: &mut HashMap<String, LLVMValueRef>,
    builder: LLVMBuilderRef,
) -> LLVMValueRef {
    match expr {
        Expr_::Byte(v) => LLVMConstInt(LLVMInt8Type(), v.into(), 0),
        Expr_::Var(v) => LLVMBuildLoad2(
            builder,
            LLVMInt8Type(),
            *variables.get_key_value(&v).unwrap().1,
            v.as_ptr() as *const _,
        ),
        Expr_::Add(a, b) => {
            let a = match_expr(a.node, variables, builder);
            let b = match_expr(b.node, variables, builder);
            LLVMBuildAdd(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Sub(a, b) => {
            let a = match_expr(a.node, variables, builder);
            let b = match_expr(b.node, variables, builder);
            LLVMBuildSub(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Mul(a, b) => {
            let a = match_expr(a.node, variables, builder);
            let b = match_expr(b.node, variables, builder);
            LLVMBuildMul(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Div(a, b) => {
            let a = match_expr(a.node, variables, builder);
            let b = match_expr(b.node, variables, builder);
            LLVMBuildUDiv(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        _ => todo!(),
    }
}
