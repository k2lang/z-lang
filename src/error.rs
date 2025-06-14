use crate::ast::Span;
use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
pub enum ZError {
    #[error("Syntax error: {message}")]
    #[diagnostic(code(z::syntax_error))]
    SyntaxError {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        message: String,
    },

    #[error("Type error: {message}")]
    #[diagnostic(code(z::type_error))]
    TypeError {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        message: String,
    },

    #[error("Name error: {message}")]
    #[diagnostic(code(z::name_error))]
    NameError {
        #[source_code]
        src: String,
        #[label("here")]
        span: SourceSpan,
        message: String,
    },

    #[error("Runtime error: {message}")]
    #[diagnostic(code(z::runtime_error))]
    RuntimeError {
        message: String,
    },

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl ZError {
    pub fn syntax_error(src: String, span: Span, message: String) -> Self {
        Self::SyntaxError {
            src,
            span: (span.start, span.end - span.start).into(),
            message,
        }
    }

    pub fn type_error(src: String, span: Span, message: String) -> Self {
        Self::TypeError {
            src,
            span: (span.start, span.end - span.start).into(),
            message,
        }
    }

    pub fn name_error(src: String, span: Span, message: String) -> Self {
        Self::NameError {
            src,
            span: (span.start, span.end - span.start).into(),
            message,
        }
    }

    pub fn runtime_error(message: String) -> Self {
        Self::RuntimeError { message }
    }
}

pub type Result<T> = std::result::Result<T, ZError>;