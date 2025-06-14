use crate::ast::{BinaryOp, Expr, Literal, Program, Span, Stmt, Type, UnaryOp};
use crate::lexer::{Span as LexerSpan, Token};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<LexerSpan>>,
    current_token: Option<LexerSpan>,
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error at position {}: {}", self.span.start, self.message)
    }
}

type Result<T> = std::result::Result<T, ParseError>;

impl Parser {
    pub fn new(tokens: Vec<LexerSpan>) -> Self {
        let mut parser = Self {
            tokens: tokens.into_iter().peekable(),
            current_token: None,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) -> Option<LexerSpan> {
        let token = self.tokens.next();
        std::mem::replace(&mut self.current_token, token)
    }

    fn peek(&mut self) -> Option<&LexerSpan> {
        self.tokens.peek()
    }

    fn expect(&mut self, expected: Token) -> Result<LexerSpan> {
        if let Some(token) = &self.current_token {
            if token.token == expected {
                Ok(self.advance().unwrap())
            } else {
                Err(ParseError {
                    message: format!("Expected {:?}, found {:?}", expected, token.token),
                    span: token.span.clone().into(),
                })
            }
        } else {
            Err(ParseError {
                message: format!("Expected {:?}, found end of file", expected),
                span: Span { start: 0, end: 0 },
            })
        }
    }

    fn parse_program(&mut self) -> Result<Program> {
        let mut statements = Vec::new();
        
        while self.current_token.is_some() {
            statements.push(self.parse_statement()?);
        }
        
        Ok(Program::new(statements))
    }

    fn parse_statement(&mut self) -> Result<Stmt> {
        match &self.current_token {
            Some(token) => match &token.token {
                Token::Let => self.parse_let_statement(),
                Token::Fn => self.parse_function_declaration(),
                Token::Return => self.parse_return_statement(),
                Token::If => {
                    let expr = self.parse_if_expression()?;
                    Ok(Stmt::Expr(expr))
                },
                Token::While => self.parse_while_statement(),
                Token::For => self.parse_for_statement(),
                Token::LeftBrace => {
                    let expr = self.parse_block_expression()?;
                    Ok(Stmt::Expr(expr))
                },
                Token::Struct => self.parse_struct_declaration(),
                Token::Import => self.parse_import_statement(),
                _ => {
                    let expr = self.parse_expression()?;
                    
                    // Check if this is an assignment
                    if let Some(token) = &self.current_token {
                        if token.token == Token::Assign {
                            let span = token.span.clone();
                            self.advance(); // Consume '='
                            let value = self.parse_expression()?;
                            return Ok(Stmt::Assign(expr, value, span.into()));
                        }
                    }
                    
                    // Consume semicolon if present
                    if let Some(token) = &self.current_token {
                        if token.token == Token::Semicolon {
                            self.advance();
                        }
                    }
                    
                    Ok(Stmt::Expr(expr))
                }
            },
            None => Err(ParseError {
                message: "Unexpected end of file".to_string(),
                span: Span { start: 0, end: 0 },
            }),
        }
    }

    // Placeholder implementations for statement parsing methods
    fn parse_let_statement(&mut self) -> Result<Stmt> {
        let let_token = self.advance().unwrap();
        let span = let_token.span.clone();
        
        // Parse identifier
        let _identifier = match &self.current_token {
            Some(token) if matches!(token.token, Token::Identifier) => {
                let id_span = token.span.clone();
                let _id = self.advance().unwrap();
                id_span
            },
            _ => return Err(ParseError {
                message: "Expected identifier after 'let'".to_string(),
                span: span.clone().into(),
            }),
        };
        
        // Parse optional type annotation
        let mut type_ann = None;
        if let Some(token) = &self.current_token {
            if token.token == Token::Colon {
                self.advance(); // Consume ':'
                type_ann = Some(self.parse_type()?);
            }
        }
        
        // Parse optional initializer
        let mut initializer = None;
        if let Some(token) = &self.current_token {
            if token.token == Token::Assign {
                self.advance(); // Consume '='
                initializer = Some(self.parse_expression()?);
            }
        }
        
        // Expect semicolon
        if let Some(token) = &self.current_token {
            if token.token == Token::Semicolon {
                self.advance();
            }
        }
        
        Ok(Stmt::Let(
            "identifier".to_string(), // Placeholder
            type_ann,
            initializer,
            span.into(),
        ))
    }

    // Placeholder implementations for other parsing methods
    fn parse_function_declaration(&mut self) -> Result<Stmt> {
        // Parse 'fn' keyword
        let fn_token = self.advance().unwrap();
        let start_pos = fn_token.span.start;
        
        // Parse function name
        let name = match &self.current_token {
            Some(token) if matches!(token.token, Token::Identifier) => {
                let name_token = self.advance().unwrap();
                // Since we don't have direct access to the source, we'll just use a placeholder name
                "main".to_string() // Placeholder for now
            },
            _ => return Err(ParseError {
                message: "Expected function name after 'fn'".to_string(),
                span: Span { start: start_pos, end: start_pos + 2 },
            }),
        };
        
        // Parse parameter list
        if let Some(token) = &self.current_token {
            if !matches!(token.token, Token::LeftParen) {
                return Err(ParseError {
                    message: "Expected '(' after function name".to_string(),
                    span: Span { start: token.span.start, end: token.span.end },
                });
            }
        } else {
            return Err(ParseError {
                message: "Expected '(' after function name".to_string(),
                span: Span { start: start_pos, end: start_pos + 2 },
            });
        }
        
        // Consume the left parenthesis
        self.advance();
        
        // For now, we'll just skip the parameter list
        let params = Vec::new();
        
        // Skip until we find the closing parenthesis
        while let Some(token) = &self.current_token {
            if matches!(token.token, Token::RightParen) {
                break;
            }
            self.advance();
        }
        
        // Expect closing parenthesis
        if let Some(token) = &self.current_token {
            if !matches!(token.token, Token::RightParen) {
                return Err(ParseError {
                    message: "Expected ')' after parameter list".to_string(),
                    span: Span { start: token.span.start, end: token.span.end },
                });
            }
        } else {
            return Err(ParseError {
                message: "Expected ')' after parameter list".to_string(),
                span: Span { start: start_pos, end: start_pos + 2 },
            });
        }
        
        // Consume the right parenthesis
        self.advance();
        
        // Parse return type (optional)
        let return_type = if let Some(token) = &self.current_token {
            if matches!(token.token, Token::Arrow) {
                self.advance(); // Consume the arrow
                
                // For now, we'll just assume it's a simple type
                if let Some(type_token) = &self.current_token {
                    if matches!(type_token.token, Token::Identifier) {
                        self.advance(); // Consume the type
                        Type::Int // Placeholder, we're not actually parsing the type yet
                    } else {
                        return Err(ParseError {
                            message: "Expected return type after '->'".to_string(),
                            span: Span { start: type_token.span.start, end: type_token.span.end },
                        });
                    }
                } else {
                    return Err(ParseError {
                        message: "Expected return type after '->'".to_string(),
                        span: Span { start: start_pos, end: start_pos + 2 },
                    });
                }
            } else {
                Type::Void
            }
        } else {
            Type::Void
        };
        
        // Parse function body
        if let Some(token) = &self.current_token {
            if !matches!(token.token, Token::LeftBrace) {
                return Err(ParseError {
                    message: "Expected '{' to begin function body".to_string(),
                    span: Span { start: token.span.start, end: token.span.end },
                });
            }
        } else {
            return Err(ParseError {
                message: "Expected '{' to begin function body".to_string(),
                span: Span { start: start_pos, end: start_pos + 2 },
            });
        }
        
        // Consume the left brace
        self.advance();
        
        // For now, we'll just create an empty block
        let body = Box::new(Stmt::Block(Vec::new(), Span { start: start_pos, end: start_pos + 2 }));
        
        // Skip until we find the closing brace
        let mut brace_count = 1;
        while let Some(token) = &self.current_token {
            if matches!(token.token, Token::LeftBrace) {
                brace_count += 1;
            } else if matches!(token.token, Token::RightBrace) {
                brace_count -= 1;
                if brace_count == 0 {
                    break;
                }
            }
            self.advance();
        }
        
        // Expect closing brace
        if let Some(token) = &self.current_token {
            if !matches!(token.token, Token::RightBrace) {
                return Err(ParseError {
                    message: "Expected '}' to end function body".to_string(),
                    span: Span { start: token.span.start, end: token.span.end },
                });
            }
        } else {
            return Err(ParseError {
                message: "Expected '}' to end function body".to_string(),
                span: Span { start: start_pos, end: start_pos + 2 },
            });
        }
        
        // Consume the right brace
        self.advance();
        
        // Create the function statement
        let end_pos = if let Some(token) = &self.current_token {
            token.span.start
        } else {
            start_pos + 10 // Just a placeholder
        };
        
        Ok(Stmt::Function(name, params, return_type, body, Span { start: start_pos, end: end_pos }))
    }

    fn parse_return_statement(&mut self) -> Result<Stmt> {
        // Placeholder implementation
        Err(ParseError {
            message: "Return statement parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt> {
        // Placeholder implementation
        Err(ParseError {
            message: "While statement parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_for_statement(&mut self) -> Result<Stmt> {
        // Placeholder implementation
        Err(ParseError {
            message: "For statement parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_struct_declaration(&mut self) -> Result<Stmt> {
        // Placeholder implementation
        Err(ParseError {
            message: "Struct declaration parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_import_statement(&mut self) -> Result<Stmt> {
        // Placeholder implementation
        Err(ParseError {
            message: "Import statement parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_if_expression(&mut self) -> Result<Expr> {
        // Placeholder implementation
        Err(ParseError {
            message: "If expression parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_block_expression(&mut self) -> Result<Expr> {
        // Placeholder implementation
        Err(ParseError {
            message: "Block expression parsing not implemented yet".to_string(),
            span: Span { start: 0, end: 0 },
        })
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        // Placeholder implementation
        self.parse_primary_expression()
    }

    fn parse_primary_expression(&mut self) -> Result<Expr> {
        match &self.current_token {
            Some(token) => {
                let span = token.span.clone();
                match &token.token {
                    Token::IntLiteral => {
                        self.advance();
                        Ok(Expr::Literal(Literal::Int(42), span.into())) // Placeholder
                    },
                    Token::FloatLiteral => {
                        self.advance();
                        Ok(Expr::Literal(Literal::Float(3.14), span.into())) // Placeholder
                    },
                    Token::StringLiteral => {
                        self.advance();
                        Ok(Expr::Literal(Literal::String("string".to_string()), span.into())) // Placeholder
                    },
                    Token::True => {
                        self.advance();
                        Ok(Expr::Literal(Literal::Bool(true), span.into()))
                    },
                    Token::False => {
                        self.advance();
                        Ok(Expr::Literal(Literal::Bool(false), span.into()))
                    },
                    Token::Null => {
                        self.advance();
                        Ok(Expr::Literal(Literal::Null, span.into()))
                    },
                    Token::Identifier => {
                        self.advance();
                        Ok(Expr::Identifier("identifier".to_string(), span.into())) // Placeholder
                    },
                    _ => Err(ParseError {
                        message: format!("Unexpected token: {:?}", token.token),
                        span: span.into(),
                    }),
                }
            },
            None => Err(ParseError {
                message: "Unexpected end of file".to_string(),
                span: Span { start: 0, end: 0 },
            }),
        }
    }

    fn parse_type(&mut self) -> Result<Type> {
        match &self.current_token {
            Some(token) => {
                match &token.token {
                    Token::Identifier => {
                        let type_name = "type".to_string(); // Placeholder
                        self.advance();
                        
                        match type_name.as_str() {
                            "int" => Ok(Type::Int),
                            "float" => Ok(Type::Float),
                            "bool" => Ok(Type::Bool),
                            "string" => Ok(Type::String),
                            "void" => Ok(Type::Void),
                            _ => Ok(Type::Struct(type_name)),
                        }
                    },
                    Token::LeftBracket => {
                        self.advance(); // Consume '['
                        let element_type = self.parse_type()?;
                        self.expect(Token::RightBracket)?; // Expect ']'
                        Ok(Type::Array(Box::new(element_type)))
                    },
                    Token::Fn => {
                        self.advance(); // Consume 'fn'
                        self.expect(Token::LeftParen)?; // Expect '('
                        
                        let mut param_types = Vec::new();
                        if self.current_token.as_ref().map(|t| t.token.clone()) != Some(Token::RightParen) {
                            loop {
                                param_types.push(self.parse_type()?);
                                
                                if self.current_token.as_ref().map(|t| t.token.clone()) == Some(Token::Comma) {
                                    self.advance(); // Consume ','
                                } else {
                                    break;
                                }
                            }
                        }
                        
                        self.expect(Token::RightParen)?; // Expect ')'
                        self.expect(Token::Arrow)?; // Expect '->'
                        let return_type = self.parse_type()?;
                        
                        Ok(Type::Function(param_types, Box::new(return_type)))
                    },
                    _ => Err(ParseError {
                        message: format!("Expected type, found {:?}", token.token),
                        span: token.span.clone().into(),
                    }),
                }
            },
            None => Err(ParseError {
                message: "Unexpected end of file while parsing type".to_string(),
                span: Span { start: 0, end: 0 },
            }),
        }
    }
}

pub fn parse(tokens: Vec<LexerSpan>) -> Result<Program> {
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}