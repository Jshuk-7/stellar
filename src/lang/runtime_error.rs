pub enum ErrorKind {
    OperatorNotDefined,
    ZeroDivision,
    TypeMismatch,
    UninitializedAccess,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OperatorNotDefined => write!(f, "Operator not defined"),
            ErrorKind::ZeroDivision => write!(f, "Division by zero"),
            ErrorKind::TypeMismatch => write!(f, "Type mismatch"),
            ErrorKind::UninitializedAccess => write!(f, "Uninitialized access"),
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
