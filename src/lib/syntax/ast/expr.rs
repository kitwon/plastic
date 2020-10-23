#[derive(Clone, Trace, Finalize, Debug, PartialEq)]
/// Javascript Expression
pub enum ExprDef {
  BinOpExpr(BinOp, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Trace, Finalize, Debug, PartialEq)]
pub struct Expr {
  pub def: ExprDef,
}

impl Expr {
  pub fn new(def: ExprDef) -> Self {
    Self { def }
  }
}
