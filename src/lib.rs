pub enum Stmt {
    Block(Vec<Stmt>),
    Empty,
    Expr(Expr),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Return(Option<Expr>),
    Var(String, Option<Expr>),
    While(Expr, Box<Stmt>),
}

pub enum Expr {
    Array(Vec<Expr>),
    Assign(String, Box<Expr>),
    Binary(BinaryOperator, Box<Expr>, Box<Expr>),
    Bool(bool),
    Call(Box<Expr>, Vec<Expr>),
    Function(Function),
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

pub struct Function {
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

pub enum UnaryOperator {
    Add,
    BitwiseNot,
    Delete,
    Not,
    Sub,
    Typeof,
    Void,
}

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
