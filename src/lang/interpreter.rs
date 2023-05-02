use super::{BinaryOp, Environment, ErrorKind, Expr, Literal, Result, RuntimeError, Stmt, UnaryOp};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InterpreterMode {
    Repl,
    Script,
}

pub struct InterpreterProperties {
    pub mode: InterpreterMode,
}

impl Default for InterpreterProperties {
    fn default() -> Self {
        Self { mode: InterpreterMode::Repl }
    }
}

pub struct Interpreter {
    environment: Environment,
    properties: InterpreterProperties,
}

impl Interpreter {
    pub fn new(properties: InterpreterProperties) -> Self {
        Self {
            environment: Environment::new(),
            properties,
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements.iter() {
            self.execute(statement.clone())
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expr(expr) => match self.evaluate(*expr) {
                Ok(result) => {
                    if let InterpreterMode::Repl = self.properties.mode {
                        crate::print_literal(result);
                    }
                }
                Err(err) => println!("Runtime Error: {err}"),
            },
            Stmt::Print(expr) => self.visit_print_statement(*expr),
            Stmt::Let(name, initializer) => self.visit_let_statement(name, initializer),
            Stmt::Block(statements) => self.visit_block_statement(&statements),
        }
    }

    fn execute_block(&mut self, statements: &[Stmt], environment: Environment) {
        let previous = self.environment.clone();

        self.environment = environment;

        for statement in statements.iter() {
            self.execute(statement.clone());
        }

        self.environment = previous;
    }

    fn visit_block_statement(&mut self, statements: &[Stmt]) {
        self.execute_block(
            statements,
            Environment::from(Box::new(self.environment.clone())),
        );
    }

    fn evaluate(&mut self, expr: Expr) -> Result<Literal> {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.visit_binary_expr(*lhs, op, *rhs),
            Expr::Grouping(expr) => self.evaluate(*expr),
            Expr::Unary(op, expr) => self.visit_unary_expr(op, *expr),
            Expr::Literal(literal) => Ok(literal),
            Expr::Variable(name) => self.visit_variable_expr(name),
            Expr::Assign(name, value) => self.visit_assign_expr(name, *value),
        }
    }

    fn visit_print_statement(&mut self, expr: Expr) {
        match self.evaluate(expr) {
            Ok(literal) => crate::print_literal(literal),
            Err(err) => println!("Runtime Error: {err}"),
        }
    }

    fn visit_let_statement(&mut self, name: String, value: Option<Box<Expr>>) {
        let initial_value = match value {
            Some(expr) => match self.evaluate(*expr) {
                Ok(literal) => Some(literal),
                Err(err) => {
                    println!("Runtime Error: {err}");
                    None
                }
            },
            None => None,
        };

        self.define_variable(name, initial_value);
    }

    fn visit_binary_expr(&mut self, lhs: Expr, op: BinaryOp, rhs: Expr) -> Result<Literal> {
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
                            return self.runtime_error(
                                ErrorKind::ZeroDivision,
                                "cannot divide by zero".to_string(),
                            );
                        }

                        Literal::Number(lvalue / rvalue)
                    }
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
            } else if let Literal::Number(rvalue) = right {
                if let BinaryOp::Add = op {
                    let res = Literal::String(lvalue + &rvalue.to_string());
                    return Ok(res);
                }
            } else if let Literal::Char(rvalue) = right {
                if let BinaryOp::Add = op {
                    let res = Literal::String(lvalue + &rvalue.to_string());
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

    fn visit_unary_expr(&mut self, op: UnaryOp, expr: Expr) -> Result<Literal> {
        let value = self.evaluate(expr)?;

        match op {
            UnaryOp::Bang => self.is_truthy(value),
            UnaryOp::Minus => self.negate(value),
        }
    }

    fn visit_variable_expr(&self, name: String) -> Result<Literal> {
        match self.environment.get(&name) {
            Some(value) => Ok(value),
            None => self.runtime_error(
                ErrorKind::UninitializedAccess,
                format!(
                    "variable '{name}' was not initialized, cannot read from unititialized memory"
                ),
            ),
        }
    }

    fn visit_assign_expr(&mut self, name: String, value: Expr) -> Result<Literal> {
        if !self.environment.contains(&name) {
            return self.runtime_error(ErrorKind::UndefinedVariable, format!("'{name}'"));
        }

        match self.evaluate(value) {
            Ok(literal) => {
                self.environment.assign(name, Some(literal.clone()));
                Ok(literal)
            }
            Err(err) => Err(err),
        }
    }

    fn define_variable(&mut self, name: String, value: Option<Literal>) {
        self.environment.define(name, value);
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
