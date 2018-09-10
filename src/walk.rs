use {Expr, Stmt};

macro_rules! walk {
    ($stmt:ident, $expr:ident, $stmt_ty:ty, $expr_ty:ty) => {
        pub fn $stmt(
            stmt: $stmt_ty,
            pre_stmt: &mut impl FnMut($stmt_ty),
            post_stmt: &mut impl FnMut($stmt_ty),
            pre_expr: &mut impl FnMut($expr_ty),
            post_expr: &mut impl FnMut($expr_ty),
        ) {
            macro_rules! r {
                ($f: ident,$a: expr) => {
                    $f($a, pre_stmt, post_stmt, pre_expr, post_expr);
                };
            }
            use Stmt::*;
            pre_stmt(stmt);
            match stmt {
                Block(stmts) => for stmt in stmts {
                    r!($stmt, stmt);
                },
                Expr(expr) => r!($expr, expr),
                If(expr, then, else_) => {
                    r!($expr, expr);
                    r!($stmt, then);
                    if let Some(else_) = else_ {
                        r!($stmt, else_);
                    }
                }
                Return(Some(expr)) => r!($expr, expr),
                Return(None) => {}
                Var(_, Some(expr)) => r!($expr, expr),
                Var(_, None) => {}
                While(expr, stmt) => {
                    r!($expr, expr);
                    r!($stmt, stmt);
                }
            }
            post_stmt(stmt);
        }
        pub fn $expr(
            expr: $expr_ty,
            pre_stmt: &mut impl FnMut($stmt_ty),
            post_stmt: &mut impl FnMut($stmt_ty),
            pre_expr: &mut impl FnMut($expr_ty),
            post_expr: &mut impl FnMut($expr_ty),
        ) {
            macro_rules! r {
                ($f: ident,$a: expr) => {
                    $f($a, pre_stmt, post_stmt, pre_expr, post_expr);
                };
            }
            use Expr::*;
            pre_expr(expr);
            match expr {
                Array(exprs) => for expr in exprs {
                    r!($expr, expr)
                },
                Assign(_, expr) => r!($expr, expr),
                Binary(_, left, right) => {
                    r!($expr, left);
                    r!($expr, right);
                }
                Bool(_) => {}
                Call(expr, args) => {
                    r!($expr, expr);
                    for arg in args {
                        r!($expr, arg)
                    }
                }
                Function(_, stmts) => {
                    for stmt in stmts {
                        r!($stmt, stmt)
                    }
                }
                Member(expr, field) => {
                    r!($expr, expr);
                    r!($expr, field);
                }
                Null => {}
                Number(_) => {}
                Object(kvs) => for (_, v) in kvs {
                    r!($expr, v);
                },
                String(_) => {}
                This => {}
                Unary(_, expr) => r!($expr, expr),
                Undefined => {}
                Var(_) => {}
            }
            post_expr(expr);
        }
    };
}

walk!(walk_stmt, walk_expr, &Stmt, &Expr);
walk!(walk_stmt_mut, walk_expr_mut, &mut Stmt, &mut Expr);
