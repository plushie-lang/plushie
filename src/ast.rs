#[derive(Debug)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Stmt {
    Cozy(Cozy),
    Snuggle(Snuggle),
    Expr(Expr),
}

#[derive(Debug)]
pub struct Cozy {
    pub ident: String,
    pub ty: Ty,
    pub expr: Expr,
}

#[derive(Debug)]
pub struct Snuggle {
    pub ident: String,
    pub stmts: Vec<Stmt>,
}

#[derive(Debug)]
pub enum Expr {
    Ribbon(String),
    Call(Call),
    Variable(String),
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
