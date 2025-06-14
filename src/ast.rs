use std::fmt;
use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    Void,
    Array(Box<Type>),
    Function(Vec<Type>, Box<Type>),
    Struct(String),
    Inferred, // For type inference
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Void => write!(f, "void"),
            Type::Array(elem_type) => write!(f, "[{}]", elem_type),
            Type::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") -> {}", ret)
            }
            Type::Struct(name) => write!(f, "{}", name),
            Type::Inferred => write!(f, "_"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal, Span),
    Identifier(String, Span),
    Binary(Box<Expr>, BinaryOp, Box<Expr>, Span),
    Unary(UnaryOp, Box<Expr>, Span),
    Call(Box<Expr>, Vec<Expr>, Span),
    Index(Box<Expr>, Box<Expr>, Span),
    Field(Box<Expr>, String, Span),
    Array(Vec<Expr>, Span),
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>, Span),
    Block(Vec<Stmt>, Span),
    Lambda(Vec<(String, Option<Type>)>, Box<Expr>, Span),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let(String, Option<Type>, Option<Expr>, Span),
    Assign(Expr, Expr, Span),
    Return(Option<Expr>, Span),
    While(Expr, Box<Stmt>, Span),
    For(String, Expr, Box<Stmt>, Span),
    Block(Vec<Stmt>, Span),
    Function(String, Vec<(String, Type)>, Type, Box<Stmt>, Span),
    Struct(String, Vec<(String, Type)>, Span),
    Import(String, Span),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Stmt>,
}

impl Program {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Self { statements }
    }
}