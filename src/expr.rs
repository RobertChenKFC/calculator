use std::ops::{Add, BitAnd, BitOr, Not, Sub};

use crate::func::{Arr, Func, FuncRef, Var};
use crate::prog::Prog;
use crate::stmt::Stmt;

pub type ValType = i8;
pub const TRUE: ValType = -1;
pub const FALSE: ValType = 0;

#[derive(Clone)]
pub enum Expr {
    Var(Var),
    Const(ValType),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Neq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Le(Box<Expr>, Box<Expr>),
    Call(FuncRef, Vec<Expr>),
    Index(Arr, Box<Expr>),
    Input,
}

pub trait ToExpr
where
    Self: Sized,
{
    fn to_expr(self) -> Expr;

    fn eq<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Eq(Box::new(self.to_expr()), Box::new(rhs.to_expr()))
    }

    fn neq<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Neq(Box::new(self.to_expr()), Box::new(rhs.to_expr()))
    }

    fn lt<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Lt(Box::new(self.to_expr()), Box::new(rhs.to_expr()))
    }

    fn le<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Le(Box::new(self.to_expr()), Box::new(rhs.to_expr()))
    }

    fn gt<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Lt(Box::new(rhs.to_expr()), Box::new(self.to_expr()))
    }

    fn ge<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Le(Box::new(rhs.to_expr()), Box::new(self.to_expr()))
    }
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

impl<Rhs: ToExpr> BitOr<Rhs> for Expr {
    type Output = Expr;
    fn bitor(self, rhs: Rhs) -> Expr {
        Expr::Or(Box::new(self), Box::new(rhs.to_expr()))
    }
}

impl Not for Expr {
    type Output = Expr;
    fn not(self) -> Expr {
        Expr::Not(Box::new(self))
    }
}

impl Expr {
    pub fn validate(&self, prog: &Prog, func: &Func, stmt: &Stmt) {
        if let Expr::Call(func_ref, args) = self {
            let func = prog.get_func(*func_ref);
            assert_eq!(args.len(), func.get_num_params());
        }
    }
}

pub fn input() -> Expr {
    Expr::Input
}
