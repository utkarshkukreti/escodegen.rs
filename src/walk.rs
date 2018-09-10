use {Expr, Stmt};

pub trait Walker {
    fn pre_stmt(&mut self, _stmt: &Stmt) {}
    fn post_stmt(&mut self, _stmt: &Stmt) {}
    fn pre_expr(&mut self, _expr: &Expr) {}
    fn post_expr(&mut self, _expr: &Expr) {}
}

pub trait WalkerMut {
    fn pre_stmt(&mut self, _stmt: &mut Stmt) {}
    fn post_stmt(&mut self, _stmt: &mut Stmt) {}
    fn pre_expr(&mut self, _expr: &mut Expr) {}
    fn post_expr(&mut self, _expr: &mut Expr) {}
}

macro_rules! walk {
    ($trait:path, $stmt:ident, $expr:ident, $stmt_ty:ty, $expr_ty:ty) => {
        pub fn $stmt<W: $trait>(walker: &mut W, stmt: $stmt_ty) {
            use Stmt::*;
            walker.pre_stmt(stmt);
            match stmt {
                Block(stmts) => for stmt in stmts {
                    $stmt(walker, stmt);
                },
                Expr(expr) => $expr(walker, expr),
                If(expr, then, else_) => {
                    $expr(walker, expr);
                    $stmt(walker, then);
                    if let Some(else_) = else_ {
                        $stmt(walker, else_);
                    }
                }
                Return(Some(expr)) => $expr(walker, expr),
                Return(None) => {}
                Var(_, Some(expr)) => $expr(walker, expr),
                Var(_, None) => {}
                While(expr, stmt) => {
                    $expr(walker, expr);
                    $stmt(walker, stmt);
                }
            }
            walker.post_stmt(stmt);
        }
        pub fn $expr<W: $trait>(walker: &mut W, expr: $expr_ty) {
            use Expr::*;
            walker.pre_expr(expr);
            match expr {
                Array(exprs) => for expr in exprs {
                    $expr(walker, expr)
                },
                Assign(_, expr) => $expr(walker, expr),
                Binary(_, left, right) => {
                    $expr(walker, left);
                    $expr(walker, right);
                }
                Bool(_) => {}
                Call(expr, args) => {
                    $expr(walker, expr);
                    for arg in args {
                        $expr(walker, arg)
                    }
                }
                Function(_, stmts) => {
                    for stmt in stmts {
                        $stmt(walker, stmt)
                    }
                }
                Member(expr, field) => {
                    $expr(walker, expr);
                    $expr(walker, field);
                }
                Null => {}
                Number(_) => {}
                Object(kvs) => for (_, v) in kvs {
                    $expr(walker, v);
                },
                String(_) => {}
                This => {}
                Unary(_, expr) => $expr(walker, expr),
                Undefined => {}
                Var(_) => {}
            }
            walker.post_expr(expr);
        }
    };
}

walk!(Walker, walk_stmt, walk_expr, &Stmt, &Expr);
walk!(
    WalkerMut,
    walk_stmt_mut,
    walk_expr_mut,
    &mut Stmt,
    &mut Expr
);
