use std::collections::HashMap;
use std::env;
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;

use llvm_sys::analysis::*;
use llvm_sys::bit_writer::*;
use llvm_sys::core::*;
use llvm_sys::execution_engine::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::*;
use tracing::debug;
use tracing::instrument;
use tracing::{error, info};

use crate::parser::{Expr, Expr_};
use crate::unwrap_null::UnwrapNull;

#[instrument(skip_all)]
pub unsafe fn compile_llvm(mut ast: Vec<Expr>) {
    let module = LLVMModuleCreateWithName(b"main\0".as_ptr() as *const _);

    let mut functions = HashMap::new();

    let last_span = ast[ast.len() - 1].span;

    ast.push(Expr {
        span: crate::lexer::Span {
            lo: last_span.lo + 1,
            hi: last_span.hi + 2,
        },
        node: Expr_::FunctionCall("_start".to_string(), vec![]),
    });

    for x in ast.clone() {
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

                let func = LLVMAddFunction(
                    module,
                    format!("{name}/0").as_bytes().as_ptr() as *const _,
                    function_sig,
                );
                functions.insert(name.clone(), func.clone());
            }
            _ => {}
        }
    }
    for x in ast {
        match x.node {
            Expr_::Define(name, args, expr) => {
                let func = *functions.get(&name).unwrap();
                let mut args_hash = HashMap::new();
                let entry = LLVMAppendBasicBlock(func, b"entry\0".as_ptr() as *const _);
                let builder = LLVMCreateBuilder();
                LLVMPositionBuilderAtEnd(builder, entry);
                for x in args.into_iter().enumerate() {
                    args_hash.insert(x.1, LLVMGetParam(func, x.0 as u32));
                }
                let mut variables = HashMap::new();
                iter_statements(
                    builder,
                    expr,
                    &mut variables,
                    &mut functions,
                    &mut args_hash,
                )
            }
            _ => {}
        }
    }
    let mut error = std::ptr::null_mut() as *mut i8;

    let mut gag = gag::BufferRedirect::stderr().unwrap();
    LLVMDumpModule(module);

    let mut output = String::new();
    gag.read_to_string(&mut output).unwrap();
    drop(gag);
    let _ = File::create("./build/pre.ll");

    fs::write("./build/pre.ll", output).expect("Unable to write file");

    LLVMVerifyModule(
        module,
        LLVMVerifierFailureAction::LLVMAbortProcessAction,
        &mut error,
    );
    LLVMDisposeMessage(error);

    let mut engine = std::ptr::null_mut();
    let mut error = std::ptr::null_mut() as *mut i8;

    LLVMLinkInMCJIT();
    LLVM_InitializeAllTargetInfos();
    LLVM_InitializeAllTargets();
    LLVM_InitializeAllTargetMCs();
    LLVM_InitializeAllAsmParsers();
    LLVM_InitializeAllAsmPrinters();

    if LLVMCreateExecutionEngineForModule(&mut engine, module, &mut error) == 1 {
        error!("failed to create execution engine");
        exit(1);
    }

    if error != std::ptr::null_mut() {
        error!("error: {:?}", error);
        LLVMDisposeMessage(error);
        exit(1);
    }

    let builddir = Path::new("./build");
    env::set_current_dir(&builddir).unwrap();

    if LLVMWriteBitcodeToFile(module, b"out.bc\0".as_ptr() as *const _) != 0 {
        error!("error writing bitcode to file, skipping");
    }
}

#[instrument(skip_all)]
unsafe fn match_expr(
    expr: Expr_,
    mut variables: &mut HashMap<String, LLVMValueRef>,
    builder: LLVMBuilderRef,
    arg: &mut HashMap<String, LLVMValueRef>,
    functions: &mut HashMap<String, LLVMValueRef>,
) -> LLVMValueRef {
    info!("{:#?}", expr);
    match expr {
        Expr_::Byte(v) => LLVMConstInt(LLVMInt8Type(), v.into(), 0),
        Expr_::Var(v) => {
            if let Some(v) = arg.get(&v) {
                *v
            } else {
                LLVMBuildLoad2(
                    builder,
                    LLVMInt8Type(),
                    *variables.get_key_value(&v).unwrap().1,
                    v.as_ptr() as *const _,
                )
            }
        }
        Expr_::Add(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildAdd(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Sub(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildSub(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Mul(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildMul(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Div(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildUDiv(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Mod(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildURem(builder, a, b, b"tmp\0".as_ptr() as *const _)
        }
        Expr_::Eq(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntEQ,
                    a,
                    b,
                    b"tmp\0".as_ptr() as *const _,
                ),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::NEq(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntNE,
                    a,
                    b,
                    b"tmp\0".as_ptr() as *const _,
                ),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::Gt(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntUGT,
                    a,
                    b,
                    b"tmp\0".as_ptr() as *const _,
                ),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::Lt(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntULT,
                    a,
                    b,
                    b"tmp\0".as_ptr() as *const _,
                ),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::EGt(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntUGE,
                    a,
                    b,
                    b"tmp\0".as_ptr() as *const _,
                ),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::ELt(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntULE,
                    a,
                    b,
                    b"tmp\0".as_ptr() as *const _,
                ),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::LNot(a) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildNot(builder, a, b"tmp\0".as_ptr() as *const _),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::LAnd(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildAnd(builder, a, b, b"tmp\0".as_ptr() as *const _),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::LOr(a, b) => {
            let a = match_expr(a.node, variables, builder, arg, functions);
            let b = match_expr(b.node, variables, builder, arg, functions);
            LLVMBuildIntCast(
                builder,
                LLVMBuildOr(builder, a, b, b"tmp\0".as_ptr() as *const _),
                LLVMInt8Type(),
                b"tmp\0".as_ptr() as *const _,
            )
        }
        Expr_::Assign(name, expr) => {
            let name = name.clone();
            let alloced = LLVMBuildAlloca(builder, LLVMInt8Type(), name.as_ptr() as *mut _);
            variables.insert(name.clone(), alloced);
            LLVMBuildStore(
                builder,
                match_expr(expr.node.clone(), &mut variables, builder, arg, functions),
                alloced,
            )
        }
        Expr_::ReAssign(name, expr) => {
            let name = name.clone();
            LLVMBuildStore(
                builder,
                match_expr(expr.node.clone(), &mut variables, builder, arg, functions),
                *variables.get_key_value(&name).unwrap().1,
            )
        }
        Expr_::FunctionCall(name, args) => {
            let a = args
                .clone()
                .into_iter()
                .map(|x| match_expr(x.node, variables, builder, arg, functions).unwrap_null())
                .collect::<Vec<LLVMValueRef>>();

            debug!("ah");
            let _ = a
                .iter()
                .map(|x| {
                    debug!("{:?}", ((*x).unwrap_null()));
                })
                .collect::<()>();
            debug!("ah");

            LLVMBuildCall2(
                builder,
                LLVMInt8Type(),
                *functions.get(&name).unwrap(),
                (a.as_ptr() as *mut LLVMValueRef).unwrap_null(),
                args.len() as u32,
                b"tmp\0".as_ptr() as *const _,
            )
        }
        _ => todo!(),
    }
}

#[instrument(skip_all)]
pub unsafe fn iter_statements(
    builder: *mut LLVMBuilder,
    expr: Vec<Expr>,
    mut variables: &mut HashMap<String, LLVMValueRef>,
    functions: &mut HashMap<String, LLVMValueRef>,
    arg: &mut HashMap<String, LLVMValueRef>,
) {
    for y in expr {
        info!("{:#?}", y.node);
        match y.node.clone() {
            Expr_::Assign(name, expr) => {
                let name = name.clone();
                let alloced = LLVMBuildAlloca(builder, LLVMInt8Type(), name.as_ptr() as *mut _);
                LLVMBuildStore(
                    builder,
                    match_expr(expr.node.clone(), &mut variables, builder, arg, functions),
                    alloced,
                );
                variables.insert(name.clone(), alloced);
            }
            Expr_::ReAssign(name, expr) => {
                let name = name.clone();
                debug!("mommy");
                let e = match_expr(expr.node.clone(), &mut variables, builder, arg, functions);
                if e.is_null() {
                    error!("Null");
                }
                debug!("mommy");
                LLVMBuildStore(builder, e, *variables.get_key_value(&name).unwrap().1);
            }
            Expr_::IfElse(expr, if_b, else_b) => {
                let condition = match_expr(expr.node, variables, builder, arg, functions);
                let condition = LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntNE,
                    condition,
                    LLVMConstInt(LLVMInt8Type(), 0, 0),
                    b"tmp\0".as_ptr() as *const _,
                );
                let func = LLVMGetBasicBlockParent(LLVMGetInsertBlock(builder));
                let then_block = LLVMAppendBasicBlock(func, b"then\0".as_ptr() as *const _);
                let else_block = LLVMAppendBasicBlock(func, b"else\0".as_ptr() as *const _);
                let end = LLVMAppendBasicBlock(func, b"end\0".as_ptr() as *const _);
                LLVMBuildCondBr(builder, condition, then_block, else_block);

                LLVMPositionBuilderAtEnd(builder, then_block);
                iter_statements(builder, if_b, variables, functions, arg);
                LLVMBuildBr(builder, end);

                LLVMPositionBuilderAtEnd(builder, else_block);
                iter_statements(builder, else_b, variables, functions, arg);
                LLVMBuildBr(builder, end);

                LLVMPositionBuilderAtEnd(builder, end);
            }
            Expr_::ForLoop(init, comp, run, block) => {
                match_expr(init.node, variables, builder, arg, functions);

                let condition = match_expr(comp.node.clone(), variables, builder, arg, functions);
                let condition = LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntNE,
                    condition,
                    LLVMConstInt(LLVMInt8Type(), 0, 0),
                    b"tmp\0".as_ptr() as *const _,
                );

                let func = LLVMGetBasicBlockParent(LLVMGetInsertBlock(builder));
                let loop_block = LLVMAppendBasicBlock(func, b"loop\0".as_ptr() as *const _);
                let end = LLVMAppendBasicBlock(func, b"end\0".as_ptr() as *const _);
                LLVMBuildCondBr(builder, condition, loop_block, end);

                LLVMPositionBuilderAtEnd(builder, loop_block);
                iter_statements(builder, block, variables, functions, arg);
                match_expr(run.node, variables, builder, arg, functions);
                let condition = match_expr(comp.node, variables, builder, arg, functions);
                let condition = LLVMBuildICmp(
                    builder,
                    LLVMIntPredicate::LLVMIntNE,
                    condition,
                    LLVMConstInt(LLVMInt8Type(), 0, 0),
                    b"tmp\0".as_ptr() as *const _,
                );
                LLVMBuildCondBr(builder, condition, loop_block, end);

                LLVMPositionBuilderAtEnd(builder, end);
            }
            Expr_::Byte(v) => {
                LLVMBuildRet(builder, LLVMConstInt(LLVMInt8Type(), v.into(), 0));
            }
            Expr_::Var(v) => {
                if let Some(v) = arg.get(&v) {
                } else {
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
            }
            Expr_::FunctionCall(name, args) => {
                LLVMBuildCall2(
                    builder,
                    LLVMInt8Type(),
                    *functions.get(&name).unwrap(),
                    args.as_ptr() as *mut _,
                    args.len() as u32,
                    b"tmp\0".as_ptr() as *const _,
                );
            }
            Expr_::Pass => {}
            _ => {
                // match_expr(y.node, variables, builder, arg, functions);
            }
        }
    }
}
