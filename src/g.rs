use {Expr, Stmt};

pub use BinaryOperator::*;
pub use UnaryOperator::*;

#[derive(Copy, Clone)]
pub struct G;

#[allow(non_upper_case_globals)]
pub static g: G = G;

impl G {
    pub fn block<I: IntoIterator<Item = Stmt>>(i: I) -> Stmt {
        Stmt::Block(i.into_iter().collect())
    }

    pub fn expr(expr: Expr) -> Stmt {
        Stmt::Expr(expr)
    }

    pub fn if_(cond: Expr, then: Stmt, else_: Option<Stmt>) -> Stmt {
        Stmt::If(cond, Box::new(then), else_.map(Box::new))
    }

    pub fn return_(expr: Option<Expr>) -> Stmt {
        Stmt::Return(expr)
    }

    pub fn let_<S: Into<String>>(name: S, expr: Option<Expr>) -> Stmt {
        Stmt::Var(name.into(), expr)
    }

    pub fn while_(expr: Expr, stmt: Stmt) -> Stmt {
        Stmt::While(expr, Box::new(stmt))
    }

    pub fn array<I: IntoIterator<Item = Expr>>(i: I) -> Expr {
        Expr::Array(i.into_iter().collect())
    }

    pub fn assign<S: Into<String>>(name: S, expr: Expr) -> Expr {
        Expr::Assign(name.into(), Box::new(expr))
    }

    pub fn binary(op: ::BinaryOperator, left: Expr, right: Expr) -> Expr {
        Expr::Binary(op, Box::new(left), Box::new(right))
    }

    pub fn bool(bool: bool) -> Expr {
        Expr::Bool(bool)
    }

    pub fn call<I: IntoIterator<Item = Expr>>(f: Expr, args: I) -> Expr {
        Expr::Call(Box::new(f), args.into_iter().collect())
    }

    pub fn function<I: IntoIterator<Item = S>, S: Into<String>>(
        params: I,
        stmts: Vec<Stmt>,
    ) -> Expr {
        Expr::Function(params.into_iter().map(Into::into).collect(), stmts)
    }

    pub fn member(expr: Expr, field: Expr) -> Expr {
        Expr::Member(Box::new(expr), Box::new(field))
    }

    pub fn null() -> Expr {
        Expr::Null
    }

    pub fn number(f64: f64) -> Expr {
        Expr::Number(f64)
    }

    pub fn object<I: IntoIterator<Item = (S, Expr)>, S: Into<String>>(kvs: I) -> Expr {
        Expr::Object(kvs.into_iter().map(|(k, v)| (k.into(), v)).collect())
    }

    pub fn string<S: Into<String>>(string: S) -> Expr {
        Expr::String(string.into())
    }

    pub fn this() -> Expr {
        Expr::This
    }

    pub fn unary(op: ::UnaryOperator, expr: Expr) -> Expr {
        Expr::Unary(op, Box::new(expr))
    }

    pub fn undefined() -> Expr {
        Expr::Undefined
    }

    pub fn var<S: Into<String>>(name: S) -> Expr {
        Expr::Var(name.into())
    }
}
