use crate::{parser::ast::Node};
use miette::{self, Diagnostic};
use thiserror::{self, Error};

#[derive(Debug, Error, Diagnostic)]
#[error("Expect token")]
#[diagnostic()]
pub struct ExpectToken(
    pub &'static str,
    pub &'static str,
    #[label("Expect `{0}` here, but found `{1}`")] pub Node,
);

#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected token")]
#[diagnostic()]
pub struct UnexpectedToken(
    pub u32,
    pub &'static str,
    #[label("line: {0} Unexpected token `{1}`")] pub Node,
);

#[derive(Debug, Error, Diagnostic)]
#[error("Unknown statement")]
#[diagnostic()]
pub struct UnknownStatement(pub &'static str, #[label("Unknown statement {0}")] pub Node);

#[derive(Debug, Error, Diagnostic)]
#[error("invalid syntax")]
#[diagnostic()]
pub struct InvalidSyntax(pub String, #[label("invalid syntax {0}")] pub Node);
