#[derive(Debug)]
pub enum FoldError {
    Overflow,
    Underflow,
    DivisionByZero,
    InvalidOperation(String),
    UnsupportedExpression(crate::ast::Expression),
    TypeMismatch { expected: String, found: String },
}

impl From<String> for FoldError {
    fn from(error: String) -> Self {
        FoldError::InvalidOperation(error)
    }
}

impl std::fmt::Display for FoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FoldError::Overflow => write!(f, "Overflow occurred during operation"),
            FoldError::Underflow => write!(f, "Underflow occurred during operation"),
            FoldError::DivisionByZero => write!(f, "Division by zero"),
            FoldError::InvalidOperation(op) => write!(f, "Invalid operation: {}", op),
            FoldError::UnsupportedExpression(expr) => {
                write!(f, "Unsupported expression: {:?}", expr)
            }
            FoldError::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "Type mismatch: expected {}, but found {}",
                    expected, found
                )
            }
        }
    }
}

impl std::error::Error for FoldError {}
