//! Formula parsing and evaluation.

use crate::cell::CellValue;

/// A parsed formula.
#[derive(Debug, Clone)]
pub struct Formula {
    /// Original formula text.
    pub text: String,
    /// Parsed expression.
    pub expr: FormulaExpr,
}

impl Formula {
    /// Parse a formula string.
    pub fn parse(text: &str) -> Result<Self, FormulaError> {
        let text = text.trim();
        if !text.starts_with('=') {
            return Err(FormulaError::InvalidSyntax(
                "Formula must start with '='".into(),
            ));
        }

        // TODO: Implement full formula parser
        Ok(Self {
            text: text.to_string(),
            expr: FormulaExpr::Value(CellValue::Number(0.0)),
        })
    }

    /// Evaluate the formula.
    pub fn evaluate(&self, _context: &FormulaContext) -> Result<CellValue, FormulaError> {
        // TODO: Implement formula evaluation
        match &self.expr {
            FormulaExpr::Value(v) => Ok(v.clone()),
            _ => Ok(CellValue::Number(0.0)),
        }
    }
}

/// Formula expression AST.
#[derive(Debug, Clone)]
pub enum FormulaExpr {
    /// Literal value.
    Value(CellValue),
    /// Cell reference.
    CellRef(crate::CellRef),
    /// Range reference.
    Range {
        start: crate::CellRef,
        end: crate::CellRef,
    },
    /// Function call.
    Function {
        name: String,
        args: Vec<FormulaExpr>,
    },
    /// Binary operation.
    BinaryOp {
        op: BinaryOp,
        left: Box<FormulaExpr>,
        right: Box<FormulaExpr>,
    },
    /// Unary operation.
    UnaryOp {
        op: UnaryOp,
        operand: Box<FormulaExpr>,
    },
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Concat,
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Percent,
}

/// Formula errors.
#[derive(Debug, Clone, thiserror::Error)]
pub enum FormulaError {
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("Division by zero")]
    DivByZero,

    #[error("Invalid reference: {0}")]
    InvalidRef(String),

    #[error("Unknown function: {0}")]
    UnknownFunction(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Circular reference")]
    CircularRef,
}

/// Formula evaluation context.
pub struct FormulaContext<'a> {
    /// Cell value lookup function.
    pub get_cell: &'a dyn Fn(crate::CellRef) -> Option<CellValue>,
}
