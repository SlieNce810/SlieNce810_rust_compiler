//! demo_compiler —— 支持双目运算符的表达式编译器（Lab0 骨架）
//!
//! 流水线：input.txt -> lexer -> parser -> semantic -> codegen(LLVM IR) -> 打印 IR
//!
//! 本工程只提供文件骨架与函数契约，实现留给你自己补。
//! 补完后运行：
//!   cargo run                     # 读取 ./input.txt，打印 LLVM IR
//!   cargo run --bin demo_compiler > out.ll && lli out.ll   # 用 lli 验证 IR 可执行

mod codegen;
mod lexer;
mod parser;
mod semantic;

use std::{ffi::CString, fs, ptr};

use llvm_sys::core::{LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildRet, LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext, LLVMDisposeBuilder, LLVMDisposeMessage, LLVMDisposeModule, LLVMFloatTypeInContext, LLVMFunctionType, LLVMModuleCreateWithNameInContext, LLVMPositionBuilderAtEnd, LLVMPrintModuleToString};

fn main() {
    // 1. 读取输入文件（内容形如 1+1）
    let input = fs::read_to_string("input.txt").expect("Failed to read file");

    // 2. 词法分析、语法分析和语义检查
    let tokens = lexer::tokenize(&input);
    let ast = parser::parse(tokens).expect("Failed to parse");
    semantic::check_semantics(&ast).expect("Semantic error");

    // 3. 创建 LLVM 上下文和模块
    // TODO(lab0-1): unsafe { LLVMContextCreate() } 建上下文；
    //               LLVMModuleCreateWithNameInContext(名字, context) 建模块。
    //               注意：CString 的 as_ptr() 必须活到调用结束（可以先绑到变量上）。
    let context = unsafe {
        LLVMContextCreate()
    };
    let module_name = CString::new("demo_compiler").unwrap();
    let module = unsafe {
        LLVMModuleCreateWithNameInContext(module_name.as_ptr(), context)
    };

    // 4. 声明 main: float ()，并追加 entry 基本块
    // TODO(lab0-2): LLVMFloatTypeInContext 取 float 类型；
    //               LLVMFunctionType(ret_ty, ptr::null_mut(), 0, 0) 造函数类型；
    //               LLVMAddFunction(module, "main", fn_ty) 声明函数；
    //               LLVMAppendBasicBlockInContext(context, function, "entry") 加基本块。
    let float_type = unsafe {
        LLVMFloatTypeInContext(context)
    };
    let function_type = unsafe {
        LLVMFunctionType(float_type, ptr::null_mut(), 0, 0)
    };
    let function_name = CString::new("main").unwrap();
    let function = unsafe {
        LLVMAddFunction(module, function_name.as_ptr(), function_type)
    };

    // 创建基本块
    let entry_name = CString::new("entry").unwrap();
    let basic_block = unsafe {
        LLVMAppendBasicBlockInContext(context, function, entry_name.as_ptr())
    };

    // 5. 生成表达式 IR，并在 entry 里 ret 出去
    // TODO(lab0-3): result = codegen::generate_llvm_ir(&ast, module, context)；
    //               再 LLVMCreateBuilderInContext + LLVMPositionBuilderAtEnd + LLVMBuildRet。
    //               注意：codegen 内部已经发射过指令，这里的位置要在基本块末尾。
    let result = codegen::generate_llvm_ir(&ast, module, context);
    // 构建返回指令
    let builder = unsafe {
        LLVMCreateBuilderInContext(context)
    };
    unsafe {
        LLVMPositionBuilderAtEnd(builder, basic_block);
        LLVMBuildRet(builder, result);
        LLVMDisposeBuilder(builder);
    }

    // 6. 打印模块
    // TODO(lab0-4): LLVMPrintModuleToString(module) -> *mut c_char，
    //               用 CStr::from_ptr(...).to_string_lossy() 转成 &str 后 println!。
    let ir_string = unsafe {
        LLVMPrintModuleToString(module)
    };
    let ir_output = unsafe {
        std::ffi::CStr::from_ptr(ir_string)
    }.to_string_lossy();
    println!("{}", ir_output);

    // 7. 清理资源
    // TODO(lab0-5): LLVMDisposeMessage(ir_string) / LLVMDisposeModule(module) /
    //               LLVMContextDispose(context)。上面任意一步提前 return 都要保证不泄漏。
    unsafe {
        LLVMDisposeMessage(ir_string);
        LLVMDisposeModule(module);
        LLVMContextDispose(context);
    }
}
