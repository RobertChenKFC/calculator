use std::ops::{Add, Sub, BitAnd};

use crate::func::{FuncRef, Var, Arr};

pub type ValType = i8;

#[derive(Clone)]
pub enum Expr {
    Var(Var),
    Const(ValType),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Call(FuncRef, Vec<Expr>),
    Index(Arr, Box<Expr>),
}

pub trait ToExpr {
    fn to_expr(self) -> Expr;
}

impl ToExpr for Expr {
    fn to_expr(self) -> Expr {
        self
    }
}

impl ToExpr for ValType {
    fn to_expr(self) -> Expr {
        Expr::Const(self)
    }
}

impl<Rhs: ToExpr> Add<Rhs> for Expr {
    type Output = Expr;
    fn add(self, rhs: Rhs) -> Self::Output {
        Expr::Add(Box::new(self), Box::new(rhs.to_expr()))
    }
}

impl<Rhs: ToExpr> Sub<Rhs> for Expr {
    type Output = Expr;
    fn sub(self, rhs: Rhs) -> Expr {
        Expr::Sub(Box::new(self), Box::new(rhs.to_expr()))
    }
}

impl<Rhs: ToExpr> BitAnd<Rhs> for Expr {
    type Output = Expr;
    fn bitand(self, rhs: Rhs) -> Expr {
        Expr::And(Box::new(self), Box::new(rhs.to_expr()))
    }
}

impl Expr {
    pub fn eq<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Eq(Box::new(self), Box::new(rhs.to_expr()))
    }
    pub fn neq<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Neq(Box::new(self), Box::new(rhs.to_expr()))
    }
    pub fn lt<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Lt(Box::new(self), Box::new(rhs.to_expr()))
    }
    pub fn le<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Le(Box::new(self), Box::new(rhs.to_expr()))
    }
}
