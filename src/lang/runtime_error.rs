pub enum ErrorKind {
    OperatorNotDefined,
    ZeroDivision,
    TypeMismatch,
    UninitializedAccess,
    UndefinedVariable,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OperatorNotDefined => write!(f, "Operator not defined"),
            ErrorKind::ZeroDivision => write!(f, "Division by zero"),
            ErrorKind::TypeMismatch => write!(f, "Type mismatch"),
            ErrorKind::UninitializedAccess => write!(f, "Uninitialized access"),
            ErrorKind::UndefinedVariable => write!(f, "Undefined variable"),
        }
    }
}

pub struct RuntimeError {
    pub what: String,
    pub kind: ErrorKind,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.what)
    }
}

pub type Result<T> = std::result::Result<T, RuntimeError>;
