use std::fmt;

#[derive(Clone, Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expr(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Return(Option<Expr>),
    Var(String, Option<Expr>),
    While(Expr, Box<Stmt>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Array(Vec<Expr>),
    Assign(String, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Bool(bool),
    Call(Box<Expr>, Vec<Expr>),
    Function(Vec<String>, Vec<Stmt>),
    Member(Box<Expr>, Box<Expr>),
    Null,
    Number(f64),
    Object(Vec<(String, Expr)>),
    String(String),
    This,
    Unary(UnaryOperator, Box<Expr>),
    Undefined,
    Var(String),
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Add,
    BitwiseNot,
    Delete,
    Not,
    Sub,
    Typeof,
    Void,
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    Add,
    And,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    Div,
    Eq,
    Eqq,
    Gt,
    Gte,
    In,
    InstanceOf,
    Lt,
    Lte,
    Mod,
    Mul,
    NotEq,
    NotEqq,
    Or,
    Shl,
    Shr,
    Sub,
    UnsignedShr,
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Stmt::*;

        match self {
            Block(stmts) => {
                write!(f, "{{")?;
                for stmt in stmts {
                    stmt.fmt(f)?;
                }
                write!(f, "}}")
            }
            Expr(expr) => {
                expr.fmt(f)?;
                write!(f, ";")
            }
            If(expr, then, else_) => {
                write!(f, "if({}){{{}}}", expr, then)?;
                if let Some(else_) = else_ {
                    write!(f, "else{{{}}}", else_)?;
                }
                Ok(())
            }
            Return(Some(expr)) => write!(f, "return {};", expr),
            Return(None) => write!(f, "return;"),
            Var(name, expr) => match expr {
                Some(expr) => write!(f, "var {}={};", name, expr),
                None => write!(f, "var {};", name),
            },
            While(expr, stmt) => write!(f, "while({}){{{}}}", expr, stmt),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Expr::*;

        match self {
            Array(exprs) => {
                write!(f, "[")?;
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", expr)?;
                }
                write!(f, "]")
            }
            Assign(name, expr) => write!(f, "{}={}", name, expr),
            Binary(op, left, right) => write!(f, "({}{}{})", left, op, right),
            Bool(bool) => write!(f, "{}", bool),
            Call(func, args) => {
                write!(f, "{}(", func)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
            Function(params, body) => {
                write!(f, "function(")?;
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, "){{")?;
                for stmt in body {
                    write!(f, "{}", stmt)?;
                }
                write!(f, "}}")
            }
            Member(expr, field) => match field.as_ref() {
                String(string) => write!(f, "{}.{}", expr, string),
                _ => write!(f, "{}[{}]", expr, field),
            },
            Null => write!(f, "null"),
            Number(f64) => write!(f, "{}", f64),
            Object(kvs) => {
                write!(f, "{{")?;
                for (i, (k, v)) in kvs.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}:{}", k, v)?;
                }
                write!(f, "}}")
            }
            String(string) => write!(f, "{:?}", string),
            This => write!(f, "this"),
            Unary(op, expr) => write!(f, "{}{}", op, expr),
            Undefined => write!(f, "undefined"),
            Var(name) => write!(f, "{}", name),
        }
    }
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use UnaryOperator::*;
        write!(
            f,
            "{}",
            match self {
                Add => "+",
                BitwiseNot => "~",
                Delete => "delete ",
                Not => "!",
                Sub => "-",
                Typeof => "typeof ",
                Void => "void ",
            }
        )
    }
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use BinaryOperator::*;
        write!(
            f,
            "{}",
            match self {
                Add => "+",
                And => "&&",
                BitwiseAnd => "&",
                BitwiseOr => "|",
                BitwiseXor => "^",
                Div => "/",
                Eq => "==",
                Eqq => "===",
                Gt => ">",
                Gte => ">=",
                In => " in ",
                InstanceOf => " instanceof ",
                Lt => "<",
                Lte => "<=",
                Mod => "%",
                Mul => "*",
                NotEq => "!=",
                NotEqq => "!==",
                Or => "||",
                Shl => "<<",
                Shr => ">>",
                Sub => "-",
                UnsignedShr => ">>>",
            }
        )
    }
}
