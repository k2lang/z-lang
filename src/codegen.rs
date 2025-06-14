use crate::ast::{BinaryOp, Expr, Literal, Program, Stmt, Type, UnaryOp};
use std::path::Path;
use std::process::Command;
use std::fs;

#[derive(Debug)]
pub struct CodegenError {
    pub message: String,
}

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Code generation error: {}", self.message)
    }
}

type Result<T> = std::result::Result<T, CodegenError>;

/// Simple code generator that outputs C code
pub struct CodeGenerator {
    indent_level: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        Self { indent_level: 0 }
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    pub fn generate(&mut self, program: Program) -> Result<String> {
        // For now, we'll generate C code instead of LLVM IR
        // This is much simpler and doesn't require LLVM dependencies
        let mut c_code = String::new();
        
        // Add standard includes
        c_code.push_str("#include <stdio.h>\n");
        c_code.push_str("#include <stdlib.h>\n");
        c_code.push_str("#include <stdbool.h>\n");
        c_code.push_str("#include <string.h>\n");
        c_code.push_str("#include <math.h>\n\n");
        
        // Add Z runtime functions
        c_code.push_str("// Z language runtime functions\n");
        
        // Print function for strings
        c_code.push_str("void print(const char* message) {\n");
        c_code.push_str("    printf(\"%s\\n\", message);\n");
        c_code.push_str("}\n\n");
        
        // Print function for integers
        c_code.push_str("void print_int(int value) {\n");
        c_code.push_str("    printf(\"%d\\n\", value);\n");
        c_code.push_str("}\n\n");
        
        // Print function for floats
        c_code.push_str("void print_float(double value) {\n");
        c_code.push_str("    printf(\"%f\\n\", value);\n");
        c_code.push_str("}\n\n");
        
        // String concatenation with integers
        c_code.push_str("char* concat_str_int(const char* str, int num) {\n");
        c_code.push_str("    char buffer[32];\n");
        c_code.push_str("    sprintf(buffer, \"%d\", num);\n");
        c_code.push_str("    char* result = malloc(strlen(str) + strlen(buffer) + 1);\n");
        c_code.push_str("    strcpy(result, str);\n");
        c_code.push_str("    strcat(result, buffer);\n");
        c_code.push_str("    return result;\n");
        c_code.push_str("}\n\n");
        
        // String concatenation with floats
        c_code.push_str("char* concat_str_float(const char* str, double num) {\n");
        c_code.push_str("    char buffer[32];\n");
        c_code.push_str("    sprintf(buffer, \"%f\", num);\n");
        c_code.push_str("    char* result = malloc(strlen(str) + strlen(buffer) + 1);\n");
        c_code.push_str("    strcpy(result, str);\n");
        c_code.push_str("    strcat(result, buffer);\n");
        c_code.push_str("    return result;\n");
        c_code.push_str("}\n\n");
        
        // Generate main function
        c_code.push_str("int main() {\n");
        self.indent_level += 1;
        
        // Generate code for each statement
        for stmt in &program.statements {
            c_code.push_str(&self.generate_statement(stmt)?);
        }
        
        // Add a default return
        c_code.push_str(&format!("{}return 0;\n", self.indent()));
        
        self.indent_level -= 1;
        c_code.push_str("}\n");
        
        Ok(c_code)
    }
    
    fn generate_statement(&mut self, stmt: &Stmt) -> Result<String> {
        match stmt {
            Stmt::Expr(expr) => {
                // Special handling for if expressions
                if let Expr::If(cond, then_branch, else_branch, _) = expr {
                    let cond_code = self.generate_expression(cond)?;
                    let mut code = format!("{}if ({}) {{\n", self.indent(), cond_code);
                    
                    self.indent_level += 1;
                    match then_branch.as_ref() {
                        Expr::Block(stmts, _) => {
                            for stmt in stmts {
                                code.push_str(&self.generate_statement(&stmt)?);
                            }
                        },
                        _ => {
                            let expr_code = self.generate_expression(then_branch)?;
                            code.push_str(&format!("{}{};\n", self.indent(), expr_code));
                        }
                    }
                    self.indent_level -= 1;
                    
                    code.push_str(&format!("{}}}", self.indent()));
                    
                    if let Some(else_branch) = else_branch {
                        code.push_str(&format!(" else {{\n"));
                        self.indent_level += 1;
                        match else_branch.as_ref() {
                            Expr::Block(stmts, _) => {
                                for stmt in stmts {
                                    code.push_str(&self.generate_statement(&stmt)?);
                                }
                            },
                            _ => {
                                let expr_code = self.generate_expression(else_branch)?;
                                code.push_str(&format!("{}{};\n", self.indent(), expr_code));
                            }
                        }
                        self.indent_level -= 1;
                        code.push_str(&format!("{}}}", self.indent()));
                    }
                    
                    code.push_str("\n");
                    return Ok(code);
                }
                
                // For other expressions
                let expr_code = self.generate_expression(expr)?;
                Ok(format!("{}{};\n", self.indent(), expr_code))
            },
            Stmt::Function(name, _params, _return_type, body, _span) => {
                // For now, we'll just handle the main function specially
                if name == "main" {
                    // We already generate the main function in the generate method
                    // So we'll just return an empty string
                    match body.as_ref() {
                        Stmt::Block(stmts, _) => {
                            let mut code = String::new();
                            for stmt in stmts {
                                code.push_str(&self.generate_statement(stmt)?);
                            }
                            Ok(code)
                        },
                        _ => Ok(String::new())
                    }
                } else {
                    // Other functions not implemented yet
                    Ok(format!("{}// Function {} not implemented yet\n", self.indent(), name))
                }
            },
            Stmt::Let(name, _type, expr, _span) => {
                let expr_code = match expr {
                    Some(e) => self.generate_expression(e)?,
                    None => "0".to_string() // Default initialization
                };
                
                // For simplicity, we'll just use C types for now
                Ok(format!("{}int {} = {};\n", self.indent(), name, expr_code))
            },
            Stmt::Assign(target, value, _span) => {
                let target_code = self.generate_expression(target)?;
                let value_code = self.generate_expression(value)?;
                Ok(format!("{}{} = {};\n", self.indent(), target_code, value_code))
            },

            Stmt::While(cond, body, _span) => {
                let cond_code = self.generate_expression(cond)?;
                let mut code = format!("{}while ({}) {{\n", self.indent(), cond_code);
                
                self.indent_level += 1;
                match body.as_ref() {
                    Stmt::Block(stmts, _) => {
                        for stmt in stmts {
                            code.push_str(&self.generate_statement(&stmt)?);
                        }
                    },
                    _ => code.push_str(&self.generate_statement(&body.as_ref())?),
                }
                self.indent_level -= 1;
                
                code.push_str(&format!("{}}}\n", self.indent()));
                Ok(code)
            },
            // For other statement types, just generate placeholder code
            _ => Ok(format!("{}// Statement not implemented yet\n", self.indent())),
        }
    }
    
    fn generate_expression(&mut self, expr: &Expr) -> Result<String> {
        match expr {
            Expr::Literal(lit, _) => {
                match lit {
                    Literal::Int(i) => Ok(i.to_string()),
                    Literal::Float(f) => Ok(f.to_string()),
                    Literal::Bool(b) => Ok(if *b { "1".to_string() } else { "0".to_string() }),
                    Literal::String(s) => Ok(format!("\"{}\"", s)),
                    Literal::Null => Ok("NULL".to_string()),
                }
            },
            Expr::Identifier(name, _) => Ok(name.clone()),
            Expr::Binary(left, op, right, _) => {
                let left_code = self.generate_expression(left)?;
                let right_code = self.generate_expression(right)?;
                
                // Special case for string concatenation
                if let BinaryOp::Add = op {
                    if left_code.starts_with("\"") && left_code.ends_with("\"") {
                        // String + something
                        if right_code.starts_with("\"") && right_code.ends_with("\"") {
                            // String + String
                            // For simplicity, we'll just use a C function to concatenate
                            let left_without_quotes = &left_code[1..left_code.len()-1];
                            let right_without_quotes = &right_code[1..right_code.len()-1];
                            let combined = format!("{}{}", left_without_quotes, right_without_quotes);
                            return Ok(format!("\"{}\"", combined));
                        } else {
                            // String + Int/Float
                            return Ok(format!("concat_str_int({}, {})", left_code, right_code));
                        }
                    }
                }
                
                let op_str = match op {
                    BinaryOp::Add => "+",
                    BinaryOp::Sub => "-",
                    BinaryOp::Mul => "*",
                    BinaryOp::Div => "/",
                    BinaryOp::Mod => "%",
                    BinaryOp::Eq => "==",
                    BinaryOp::Neq => "!=",
                    BinaryOp::Lt => "<",
                    BinaryOp::Lte => "<=",
                    BinaryOp::Gt => ">",
                    BinaryOp::Gte => ">=",
                    BinaryOp::And => "&&",
                    BinaryOp::Or => "||",
                };
                
                Ok(format!("({} {} {})", left_code, op_str, right_code))
            },
            Expr::Unary(op, expr, _) => {
                let expr_code = self.generate_expression(expr)?;
                
                let op_str = match op {
                    UnaryOp::Neg => "-",
                    UnaryOp::Not => "!",
                };
                
                Ok(format!("({}{})", op_str, expr_code))
            },
            Expr::Call(func, args, _) => {
                let func_code = self.generate_expression(func)?;
                
                let mut args_code = Vec::new();
                for arg in args {
                    args_code.push(self.generate_expression(arg)?);
                }
                
                Ok(format!("{}({})", func_code, args_code.join(", ")))
            },
            // For now, just generate placeholder code for other expressions
            _ => Ok("/* Expression not implemented yet */".to_string()),
        }
    }
}

pub fn generate_ir(program: Program) -> Result<String> {
    let mut codegen = CodeGenerator::new();
    codegen.generate(program)
}

pub fn generate_executable(code: &str, output_path: &Path) -> Result<()> {
    // Write C code to a temporary file
    let temp_dir = std::env::temp_dir();
    let c_path = temp_dir.join("z_program.c");
    
    fs::write(&c_path, code).map_err(|e| CodegenError {
        message: format!("Failed to write C code to file: {}", e),
    })?;
    
    // Compile C code to executable using GCC or Clang
    let compiler = if Command::new("gcc").arg("--version").status().is_ok() {
        "gcc"
    } else if Command::new("clang").arg("--version").status().is_ok() {
        "clang"
    } else {
        return Err(CodegenError {
            message: "Neither GCC nor Clang found. Please install a C compiler.".to_string(),
        });
    };
    
    // Add optimization flags for maximum performance
    let status = Command::new(compiler)
        .arg("-O3")                // Maximum optimization
        .arg("-march=native")      // Optimize for current CPU
        .arg("-flto")              // Link-time optimization
        .arg("-c")                 // Compile only
        .arg(&c_path)
        .arg("-o")
        .arg(temp_dir.join("z_program.o"))
        .status()
        .map_err(|e| CodegenError {
            message: format!("Failed to execute {}: {}", compiler, e),
        })?;
    
    if !status.success() {
        return Err(CodegenError {
            message: format!("{} compilation failed", compiler),
        });
    }
    
    // Link the object file
    let status = Command::new(compiler)
        .arg("-O3")
        .arg("-march=native")
        .arg("-flto")
        .arg(temp_dir.join("z_program.o"))
        .arg("-o")
        .arg(output_path)
        .arg("-lm")               // Link math library
        .status()
        .map_err(|e| CodegenError {
            message: format!("Failed to link: {}", e),
        })?;
    
    if !status.success() {
        return Err(CodegenError {
            message: "Linking failed".to_string(),
        });
    }
    
    // Clean up temporary files
    let _ = fs::remove_file(c_path);
    let _ = fs::remove_file(temp_dir.join("z_program.o"));
    
    Ok(())
}