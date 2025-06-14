use logos::Logos;
use std::fmt;
use std::ops::Range;

#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    #[token("fn")]
    Fn,
    
    #[token("let")]
    Let,
    
    #[token("if")]
    If,
    
    #[token("else")]
    Else,
    
    #[token("while")]
    While,
    
    #[token("for")]
    For,
    
    #[token("return")]
    Return,
    
    #[token("struct")]
    Struct,
    
    #[token("enum")]
    Enum,
    
    #[token("match")]
    Match,
    
    #[token("import")]
    Import,
    
    #[token("true")]
    True,
    
    #[token("false")]
    False,
    
    #[token("null")]
    Null,
    
    // Operators
    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,
    
    #[token("*")]
    Star,
    
    #[token("/")]
    Slash,
    
    #[token("%")]
    Percent,
    
    #[token("=")]
    Assign,
    
    #[token("==")]
    Equal,
    
    #[token("!=")]
    NotEqual,
    
    #[token("<")]
    Less,
    
    #[token("<=")]
    LessEqual,
    
    #[token(">")]
    Greater,
    
    #[token(">=")]
    GreaterEqual,
    
    #[token("&&")]
    And,
    
    #[token("||")]
    Or,
    
    #[token("!")]
    Not,
    
    // Delimiters
    #[token("(")]
    LeftParen,
    
    #[token(")")]
    RightParen,
    
    #[token("{")]
    LeftBrace,
    
    #[token("}")]
    RightBrace,
    
    #[token("[")]
    LeftBracket,
    
    #[token("]")]
    RightBracket,
    
    #[token(";")]
    Semicolon,
    
    #[token(",")]
    Comma,
    
    #[token(".")]
    Dot,
    
    #[token(":")]
    Colon,
    
    #[token("->")]
    Arrow,
    
    // Literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    
    #[regex(r"[0-9]+")]
    IntLiteral,
    
    #[regex(r"[0-9]+\.[0-9]+")]
    FloatLiteral,
    
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    
    // Comments and whitespace
    #[regex(r"//.*", logos::skip)]
    Comment,
    
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    MultiLineComment,
    
    #[regex(r"[ \t\n\r]+", logos::skip)]
    Whitespace,
    
    // Error handling - Logos 0.13+ doesn't need #[error] anymore
    Error,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Identifier => write!(f, "identifier"),
            Token::IntLiteral => write!(f, "integer literal"),
            Token::FloatLiteral => write!(f, "float literal"),
            Token::StringLiteral => write!(f, "string literal"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub token: Token,
    pub span: Range<usize>,
}

pub struct LexerError {
    pub message: String,
    pub span: Range<usize>,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexer error at position {}: {}", self.span.start, self.message)
    }
}

pub fn lex(source: &str) -> Result<Vec<Span>, LexerError> {
    let mut lexer = Token::lexer(source);
    let mut tokens = Vec::new();
    
    while let Some(token) = lexer.next() {
        let span = lexer.span();
        match token {
            Ok(token) => tokens.push(Span { token, span }),
            Err(_) => {
                return Err(LexerError {
                    message: format!("Invalid token: '{}'", &source[span.clone()]),
                    span,
                });
            }
        }
    }
    
    Ok(tokens)
}