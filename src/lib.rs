mod lexer;
mod parser;
mod ast;
mod codegen;
mod error;
mod typechecker;
mod optimizer;

use std::path::Path;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Lexer error: {0}")]
    LexerError(String),
    
    #[error("Parser error: {0}")]
    ParserError(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Code generation error: {0}")]
    CodegenError(String),
}

pub type Result<T> = std::result::Result<T, CompilerError>;

/// Compiles a Z source file to an executable
pub fn compile_file(input: &Path, output: &Path, opt_level: u8) -> Result<()> {
    // Read the source file
    let source = fs::read_to_string(input)?;
    
    // Lexical analysis
    let tokens = lexer::lex(&source)
        .map_err(|e| CompilerError::LexerError(e.to_string()))?;
    
    // Parsing
    let ast = parser::parse(tokens)
        .map_err(|e| CompilerError::ParserError(e.to_string()))?;
    
    // Type checking
    let typed_ast = typechecker::typecheck(ast)
        .map_err(|e| CompilerError::TypeError(e.to_string()))?;
    
    // Code generation
    let ir = codegen::generate_ir(typed_ast)
        .map_err(|e| CompilerError::CodegenError(e.to_string()))?;
    
    // Optimization
    let optimized_ir = optimizer::optimize(ir, opt_level)
        .map_err(|e| CompilerError::CodegenError(e.to_string()))?;
    
    // Generate executable
    codegen::generate_executable(&optimized_ir, output)
        .map_err(|e| CompilerError::CodegenError(e.to_string()))?;
    
    Ok(())
}

use std::time::{Duration, Instant};

/// Runs a Z source file directly
pub fn run_file(input: &Path) -> Result<()> {
    println!("Z Compiler - The fastest programming language ever!");
    println!("----------------------------------------------------");
    
    // Start timing the compilation process
    let compilation_start = Instant::now();
    
    // Read the source file
    let source = fs::read_to_string(input)?;
    println!("Source file: {}", input.display());
    
    // Lexical analysis
    let lexer_start = Instant::now();
    let tokens = lexer::lex(&source)
        .map_err(|e| CompilerError::LexerError(e.to_string()))?;
    let lexer_time = lexer_start.elapsed();
    println!("Lexical analysis: {:?}", lexer_time);
    
    // Parsing
    let parser_start = Instant::now();
    let ast = parser::parse(tokens)
        .map_err(|e| CompilerError::ParserError(e.to_string()))?;
    let parser_time = parser_start.elapsed();
    println!("Parsing: {:?}", parser_time);
    
    // Type checking
    let typecheck_start = Instant::now();
    let typed_ast = typechecker::typecheck(ast)
        .map_err(|e| CompilerError::TypeError(e.to_string()))?;
    let typecheck_time = typecheck_start.elapsed();
    println!("Type checking: {:?}", typecheck_time);
    
    // Code generation
    let codegen_start = Instant::now();
    let c_code = codegen::generate_ir(typed_ast)
        .map_err(|e| CompilerError::CodegenError(e.to_string()))?;
    let codegen_time = codegen_start.elapsed();
    println!("Code generation: {:?}", codegen_time);
    
    // Create a temporary output file
    let temp_dir = std::env::temp_dir();
    let c_file = temp_dir.join("z_temp_program.c");
    let output = temp_dir.join("z_temp_executable");
    
    // Write the C code to a file
    fs::write(&c_file, &c_code)?;
    
    // Compile the C code
    let compiler = if std::process::Command::new("gcc").arg("--version").status().is_ok() {
        "gcc"
    } else if std::process::Command::new("clang").arg("--version").status().is_ok() {
        "clang"
    } else {
        return Err(CompilerError::CodegenError(
            "Neither GCC nor Clang found. Please install a C compiler.".to_string()
        ));
    };
    
    // Compile the C code to an executable with maximum optimization
    println!("Compiling with {}", compiler);
    let native_compile_start = Instant::now();
    let compile_status = std::process::Command::new(compiler)
        .arg("-O3")                // Maximum optimization
        .arg("-march=native")      // Optimize for current CPU
        .arg("-flto")              // Link-time optimization
        .arg("-o")
        .arg(&output)
        .arg(&c_file)
        .arg("-lm")                // Link math library
        .status()?;
    let native_compile_time = native_compile_start.elapsed();
    println!("Native compilation: {:?}", native_compile_time);
    
    if !compile_status.success() {
        return Err(CompilerError::CodegenError(
            format!("Compilation failed with status: {}", compile_status)
        ));
    }
    
    // Total compilation time
    let total_compilation_time = compilation_start.elapsed();
    println!("Total compilation time: {:?}", total_compilation_time);
    println!("\n----------------------------------------------------");
    
    // Execute the compiled program
    println!("Program output:");
    println!("----------------------------------------------------");
    
    let execution_start = Instant::now();
    let output_result = std::process::Command::new(output.clone())
        .output()?;
    let execution_time = execution_start.elapsed();
    
    // Print the output
    println!("{}", String::from_utf8_lossy(&output_result.stdout));
    println!("----------------------------------------------------");
    println!("Execution time: {:?}", execution_time);
    
    // Clean up the temporary files
    let _ = fs::remove_file(c_file);
    let _ = fs::remove_file(output);
    
    if !output_result.status.success() {
        return Err(CompilerError::CodegenError(
            format!("Program exited with status: {}", output_result.status)
        ));
    }
    
    Ok(())
}