use crate::ast::{BinaryOp, Expr, Literal, Program, Stmt, Type, UnaryOp};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TypeError {
    pub message: String,
    pub span: Option<crate::ast::Span>,
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.span {
            Some(span) => write!(f, "Type error at position {}: {}", span.start, self.message),
            None => write!(f, "Type error: {}", self.message),
        }
    }
}

type Result<T> = std::result::Result<T, TypeError>;

pub struct TypeChecker {
    // Symbol table for variables and their types
    variables: HashMap<String, Type>,
    // Symbol table for functions
    functions: HashMap<String, (Vec<Type>, Type)>,
    // Symbol table for structs
    structs: HashMap<String, HashMap<String, Type>>,
    // Current return type for function checking
    current_return_type: Option<Type>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            current_return_type: None,
        }
    }

    pub fn check_program(&mut self, program: Program) -> Result<Program> {
        // First pass: register all function and struct declarations
        for stmt in &program.statements {
            match stmt {
                Stmt::Function(name, params, return_type, _, _) => {
                    let param_types: Vec<Type> = params.iter().map(|(_, ty)| ty.clone()).collect();
                    self.functions.insert(name.clone(), (param_types, return_type.clone()));
                }
                Stmt::Struct(name, fields, _) => {
                    let mut field_types = HashMap::new();
                    for (field_name, field_type) in fields {
                        field_types.insert(field_name.clone(), field_type.clone());
                    }
                    self.structs.insert(name.clone(), field_types);
                }
                _ => {}
            }
        }

        // Second pass: check all statements
        let mut checked_statements = Vec::new();
        for stmt in program.statements {
            checked_statements.push(self.check_statement(stmt)?);
        }

        Ok(Program::new(checked_statements))
    }

    fn check_statement(&mut self, stmt: Stmt) -> Result<Stmt> {
        match stmt {
            Stmt::Let(name, type_ann, initializer, span) => {
                let var_type = match (&type_ann, &initializer) {
                    (Some(ty), _) => ty.clone(),
                    (None, Some(expr)) => {
                        let (_checked_expr, expr_type) = self.check_expression(expr.clone())?;
                        expr_type
                    }
                    (None, None) => {
                        return Err(TypeError {
                            message: "Cannot infer type for variable without initializer".to_string(),
                            span: Some(span.clone()),
                        });
                    }
                };

                // Check that initializer matches the declared type
                let checked_initializer = if let Some(init) = initializer {
                    let (checked_init, init_type) = self.check_expression(init)?;
                    if let Some(ty) = &type_ann {
                        self.check_type_compatibility(init_type, ty.clone(), &span)?;
                    }
                    Some(checked_init)
                } else {
                    None
                };

                // Add variable to symbol table
                self.variables.insert(name.clone(), var_type.clone());

                Ok(Stmt::Let(name, type_ann, checked_initializer, span))
            }
            // Placeholder implementations for other statement types
            _ => Ok(stmt),
        }
    }

    fn check_expression(&mut self, expr: Expr) -> Result<(Expr, Type)> {
        match expr {
            Expr::Literal(lit, span) => {
                let ty = match lit {
                    Literal::Int(_) => Type::Int,
                    Literal::Float(_) => Type::Float,
                    Literal::Bool(_) => Type::Bool,
                    Literal::String(_) => Type::String,
                    Literal::Null => Type::Void,
                };
                Ok((Expr::Literal(lit, span), ty))
            }
            Expr::Identifier(name, span) => {
                if let Some(ty) = self.variables.get(&name) {
                    Ok((Expr::Identifier(name, span), ty.clone()))
                } else {
                    Err(TypeError {
                        message: format!("Undefined variable: {}", name),
                        span: Some(span),
                    })
                }
            }
            Expr::Binary(left, op, right, span) => {
                let (checked_left, left_type) = self.check_expression(*left)?;
                let (checked_right, right_type) = self.check_expression(*right)?;
                
                let result_type = self.check_binary_op(&op, &left_type, &right_type, &span)?;
                
                Ok((
                    Expr::Binary(Box::new(checked_left), op, Box::new(checked_right), span),
                    result_type,
                ))
            }
            // Placeholder implementations for other expression types
            _ => Ok((expr, Type::Inferred)),
        }
    }

    fn check_binary_op(&self, op: &BinaryOp, left_type: &Type, right_type: &Type, span: &crate::ast::Span) -> Result<Type> {
        match op {
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                if left_type == &Type::Int && right_type == &Type::Int {
                    Ok(Type::Int)
                } else if left_type == &Type::Float && right_type == &Type::Float {
                    Ok(Type::Float)
                } else if left_type == &Type::Int && right_type == &Type::Float 
                      || left_type == &Type::Float && right_type == &Type::Int {
                    Ok(Type::Float)
                } else if op == &BinaryOp::Add && (left_type == &Type::String || right_type == &Type::String) {
                    Ok(Type::String)
                } else {
                    Err(TypeError {
                        message: format!("Invalid operand types for binary operator: {:?} and {:?}", left_type, right_type),
                        span: Some(span.clone()),
                    })
                }
            }
            BinaryOp::Eq | BinaryOp::Neq => {
                // Most types can be compared for equality
                Ok(Type::Bool)
            }
            BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
                if (left_type == &Type::Int && right_type == &Type::Int)
                    || (left_type == &Type::Float && right_type == &Type::Float)
                    || (left_type == &Type::Int && right_type == &Type::Float)
                    || (left_type == &Type::Float && right_type == &Type::Int)
                {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError {
                        message: format!("Invalid operand types for comparison operator: {:?} and {:?}", left_type, right_type),
                        span: Some(span.clone()),
                    })
                }
            }
            BinaryOp::And | BinaryOp::Or => {
                if left_type == &Type::Bool && right_type == &Type::Bool {
                    Ok(Type::Bool)
                } else {
                    Err(TypeError {
                        message: format!("Invalid operand types for logical operator: {:?} and {:?}", left_type, right_type),
                        span: Some(span.clone()),
                    })
                }
            }
        }
    }

    fn check_type_compatibility(&self, actual: Type, expected: Type, span: &crate::ast::Span) -> Result<()> {
        if actual == expected {
            Ok(())
        } else {
            Err(TypeError {
                message: format!("Type mismatch: expected {:?}, found {:?}", expected, actual),
                span: Some(span.clone()),
            })
        }
    }
}

pub fn typecheck(program: Program) -> Result<Program> {
    let mut typechecker = TypeChecker::new();
    typechecker.check_program(program)
}