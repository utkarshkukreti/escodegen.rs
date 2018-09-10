use {Expr, Stmt};

pub trait Walker {
    fn pre_stmt(&mut self, _stmt: &mut Stmt) {}
    fn post_stmt(&mut self, _stmt: &mut Stmt) {}
    fn pre_expr(&mut self, _expr: &mut Expr) {}
    fn post_expr(&mut self, _expr: &mut Expr) {}
}

pub fn walk_stmt<W: Walker>(walker: &mut W, stmt: &mut Stmt) {
    use Stmt::*;
    walker.pre_stmt(stmt);
    match stmt {
        Block(stmts) => for stmt in stmts {
            walk_stmt(walker, stmt);
        },
        Expr(expr) => walk_expr(walker, expr),
        If(expr, then, else_) => {
            walk_expr(walker, expr);
            walk_stmt(walker, then);
            if let Some(else_) = else_ {
                walk_stmt(walker, else_);
            }
        }
        Return(Some(expr)) => walk_expr(walker, expr),
        Return(None) => {}
        Var(_, Some(expr)) => walk_expr(walker, expr),
        Var(_, None) => {}
        While(expr, stmt) => {
            walk_expr(walker, expr);
            walk_stmt(walker, stmt);
        }
    }
    walker.post_stmt(stmt);
}

pub fn walk_expr<W: Walker>(walker: &mut W, expr: &mut Expr) {
    use Expr::*;
    walker.pre_expr(expr);
    match expr {
        Array(exprs) => for expr in exprs {
            walk_expr(walker, expr)
        },
        Assign(_, expr) => walk_expr(walker, expr),
        Binary(_, left, right) => {
            walk_expr(walker, left);
            walk_expr(walker, right);
        }
        Bool(_) => {}
        Call(expr, args) => {
            walk_expr(walker, expr);
            for arg in args {
                walk_expr(walker, arg)
            }
        }
        Function(_, stmts) => {
            for stmt in stmts {
                walk_stmt(walker, stmt)
            }
        }
        Member(expr, field) => {
            walk_expr(walker, expr);
            walk_expr(walker, field);
        }
        Null => {}
        Number(_) => {}
        Object(kvs) => for (_, v) in kvs {
            walk_expr(walker, v);
        },
        String(_) => {}
        This => {}
        Unary(_, expr) => walk_expr(walker, expr),
        Undefined => {}
        Var(_) => {}
    }
    walker.post_expr(expr);
}
