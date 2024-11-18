#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Decl(Decl),
    Func(Func),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Decl {
    pub ident: String,
    pub ty: Ty,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Func {
    pub ident: String,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Expr {
    Ribbon(String),
    Call(Call),
    Cozy(String),
}

#[derive(Debug)]
pub struct Call {
    pub ident: String,
    pub args: Vec<Expr>,
}

#[derive(Debug)]
pub enum Ty {
    Ribbon,
    Bunny,
}
