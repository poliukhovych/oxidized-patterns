pub mod ast {
    #[derive(Debug)]
    pub enum Stmt {
        Expr(Box<Expr>),
        Let(Box<Name>, Box<Expr>),
    }

    #[derive(Debug)]
    pub struct Name {
        pub value: String,
    }

    #[derive(Debug)]
    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

pub trait Folder {
    fn fold_name(&mut self, n: Box<ast::Name>) -> Box<ast::Name> {
        n
    }

    fn fold_expr(&mut self, e: Box<ast::Expr>) -> Box<ast::Expr> {
        use ast::Expr;
        match *e {
            Expr::IntLit(_) => e,
            Expr::Add(left, right) => Box::new(Expr::Add(
                self.fold_expr(left),
                self.fold_expr(right),
            )),
            Expr::Sub(left, right) => Box::new(Expr::Sub(
                self.fold_expr(left),
                self.fold_expr(right),
            )),
        }
    }

    fn fold_stmt(&mut self, s: Box<ast::Stmt>) -> Box<ast::Stmt> {
        use ast::Stmt;
        match *s {
            Stmt::Expr(e) => Box::new(Stmt::Expr(self.fold_expr(e))),
            Stmt::Let(n, e) => Box::new(Stmt::Let(
                self.fold_name(n),
                self.fold_expr(e),
            )),
        }
    }
}

pub struct Renamer;

impl Folder for Renamer {
    fn fold_name(&mut self, _n: Box<ast::Name>) -> Box<ast::Name> {
        Box::new(ast::Name { value: "foo".to_string() })
    }
}

pub fn demo() {
    use ast::*;
    let stmt = Stmt::Let(
        Box::new(Name { value: "x".into() }),
        Box::new(Expr::Add(
            Box::new(Expr::IntLit(1)),
            Box::new(Expr::Sub(
                Box::new(Expr::IntLit(5)),
                Box::new(Expr::IntLit(2)),
            )),
        )),
    );
    println!("[Fold AST demo] Original: {:?}", stmt);
    let mut renamer = Renamer;
    let new_stmt = renamer.fold_stmt(Box::new(stmt));
    println!("[Fold AST demo] Renamed: {:?}", new_stmt);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ast::*;

    #[test]
    fn test_renamer_on_let() {
        let stmt = Stmt::Let(
            Box::new(Name { value: "x".into() }),
            Box::new(Expr::IntLit(3)),
        );
        let mut renamer = Renamer;
        if let Stmt::Let(n, e) = *renamer.fold_stmt(Box::new(stmt)) {
            assert_eq!(n.value, "foo");
            if let Expr::IntLit(v) = *e {
                assert_eq!(v, 3);
            } else {
                panic!("Expected IntLit");
            }
        } else {
            panic!("Expected Let statement");
        }
    }

    #[test]
    fn test_renamer_on_nested_expr() {
        let stmt = Stmt::Expr(Box::new(Expr::Add(
            Box::new(Expr::IntLit(2)),
            Box::new(Expr::Sub(
                Box::new(Expr::IntLit(4)),
                Box::new(Expr::IntLit(1)),
            )),
        )));
        let mut renamer = Renamer;
        let res = renamer.fold_stmt(Box::new(stmt));
        // Only names are renamed, expressions unchanged
        if let ast::Stmt::Expr(e) = *res {
            // Should still be the same structure
            assert!(matches!(*e, Expr::Add(_, _)));
        } else {
            panic!("Expected Expr statement");
        }
    }
}
