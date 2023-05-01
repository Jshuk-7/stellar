use super::{BinaryOp, Expr, Literal, UnaryOp};

pub enum ErrorKind {
    OperatorNotDefined,
    ZeroDivision,
    TypeMismatch,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::OperatorNotDefined => write!(f, "Operator not defined"),
            ErrorKind::ZeroDivision => write!(f, "Division by zero"),
            ErrorKind::TypeMismatch => write!(f, "Type mismatch"),
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

#[derive(Default)]
pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&self, expr: Expr) -> Result<Literal> {
        self.evaluate(expr)
    }

    fn evaluate(&self, expr: Expr) -> Result<Literal> {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.visit_binary_expr(*lhs, op, *rhs),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Unary(op, expr) => self.visit_unary_expr(op, *expr),
            Expr::Literal(literal) => Ok(literal),
        }
    }

    fn visit_binary_expr(&self, lhs: Expr, op: BinaryOp, rhs: Expr) -> Result<Literal> {
        let left = self.evaluate(lhs)?;
        let right = self.evaluate(rhs)?;

        if let Literal::Number(lvalue) = left {
            if let Literal::Number(rvalue) = right {
                let res = match op {
                    BinaryOp::Equal => Literal::Bool(lvalue == rvalue),
                    BinaryOp::NotEqual => Literal::Bool(lvalue != rvalue),
                    BinaryOp::Add => Literal::Number(lvalue + rvalue),
                    BinaryOp::Sub => Literal::Number(lvalue - rvalue),
                    BinaryOp::Mul => Literal::Number(lvalue * rvalue),
                    BinaryOp::Div => {
                        if rvalue == 0.0 {
                            return self.runtime_error(ErrorKind::ZeroDivision, "cannot divide by zero".to_string());
                        }

                        Literal::Number(lvalue / rvalue)
                    },
                    BinaryOp::Gt => Literal::Bool(lvalue > rvalue),
                    BinaryOp::Gte => Literal::Bool(lvalue >= rvalue),
                    BinaryOp::Lt => Literal::Bool(lvalue < rvalue),
                    BinaryOp::Lte => Literal::Bool(lvalue <= rvalue),
                };

                return Ok(res);
            }
        } else if let Literal::String(lvalue) = left.clone() {
            if let Literal::String(rvalue) = right.clone() {
                if let BinaryOp::Equal = op {
                    let res = Literal::Bool(lvalue == rvalue);
                    return Ok(res);
                } else if let BinaryOp::NotEqual = op {
                    let res = Literal::Bool(lvalue != rvalue);
                    return Ok(res);
                } else if let BinaryOp::Add = op {
                    let res = Literal::String(lvalue + &rvalue);
                    return Ok(res);
                }
            }
        } else if let Literal::Bool(lvalue) = left {
            if let Literal::Bool(rvalue) = right {
                if let BinaryOp::Equal = op {
                    let res = Literal::Bool(lvalue == rvalue);
                    return Ok(res);
                } else if let BinaryOp::NotEqual = op {
                    let res = Literal::Bool(lvalue != rvalue);
                    return Ok(res);
                }
            }
        } else if let Literal::Char(lvalue) = left {
            if let Literal::Char(rvalue) = right {
                if let BinaryOp::Equal = op {
                    let res = Literal::Bool(lvalue == rvalue);
                    return Ok(res);
                } else if let BinaryOp::NotEqual = op {
                    let res = Literal::Bool(lvalue != rvalue);
                    return Ok(res);
                }
            }
        }

        let typename1 = self.typename_from_literal(left);
        let typename2 = self.typename_from_literal(right);
        self.runtime_error(
            ErrorKind::OperatorNotDefined,
            format!("'{op:?}' not supported for types '{typename1}' and '{typename2}'"),
        )
    }

    fn visit_unary_expr(&self, op: UnaryOp, expr: Expr) -> Result<Literal> {
        let value = self.evaluate(expr)?;

        match op {
            UnaryOp::Bang => self.is_truthy(value),
            UnaryOp::Minus => self.negate(value),
        }
    }

    fn is_truthy(&self, value: Literal) -> Result<Literal> {
        let res = match value {
            Literal::Number(x) => Literal::Bool(x > 0.0),
            Literal::String(x) => Literal::Bool(!x.is_empty()),
            Literal::Bool(x) => Literal::Bool(x),
            Literal::Char(x) => Literal::Bool(x != '0'),
            Literal::Null => Literal::Bool(false),
        };

        Ok(res)
    }

    fn negate(&self, value: Literal) -> Result<Literal> {
        if let Literal::Number(x) = value {
            let res = Literal::Number(-x);
            return Ok(res);
        }

        let typename = self.typename_from_literal(value);
        self.runtime_error(
            ErrorKind::OperatorNotDefined,
            format!("unary negate not supported for type '{typename}'"),
        )
    }

    fn typename_from_literal(&self, literal: Literal) -> &str {
        match literal {
            Literal::Number(..) => "number",
            Literal::String(..) => "String",
            Literal::Bool(..) => "bool",
            Literal::Char(..) => "char",
            Literal::Null => "null",
        }
    }

    fn runtime_error(&self, kind: ErrorKind, msg: String) -> Result<Literal> {
        Err(RuntimeError { what: msg, kind })
    }
}
