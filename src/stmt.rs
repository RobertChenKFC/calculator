use crate::expr::{Expr, ToExpr, ValType};
use crate::func::{Var, Arr};
use crate::reference::CallbackRef;

pub enum Stmt {
    If(Expr, Vec<Stmt>),
    Return(Expr),
    Assign(Var, Expr),
    AssignArr(Arr, Expr, Expr),
    Do(Expr),
    While(Expr, Vec<Stmt>),
    Debug(&'static str, Var),
    Check(Var, CallbackRef),
}

pub trait ToStmt {
    fn to_stmt(self) -> Stmt;
}

impl ToStmt for Stmt {
    fn to_stmt(self) -> Stmt {
        self
    }
}

impl ToStmt for Expr {
    fn to_stmt(self) -> Stmt {
        Stmt::Do(self)
    }
}

#[macro_export]
macro_rules! if_ {
    ($cond:expr => {$($stmt:expr);*$(;)?}) => { Stmt::If(($cond).to_expr(), vec![$($stmt),*]) }
}

pub fn return_<T: ToExpr>(expr: T) -> Stmt {
    Stmt::Return(expr.to_expr())
}

pub trait ToAssignStmt {
    fn to_assign_stmt(self, rhs: Expr) -> Stmt;
}

impl ToAssignStmt for Var {
    fn to_assign_stmt(self, rhs: Expr) -> Stmt {
        Stmt::Assign(self, rhs)
    }
}

pub fn let_<L: ToAssignStmt, R: ToExpr>(lhs: L, rhs: R) -> Stmt {
    lhs.to_assign_stmt(rhs.to_expr())
}

#[macro_export]
macro_rules! while_ {
    ($cond:expr => {$($stmt:expr);*$(;)?}) => { Stmt::While(($cond).to_expr(), vec![$(($stmt).to_stmt()),*]) }
}

pub fn debug_(message: &'static str, var: Var) -> Stmt {
    Stmt::Debug(message, var)
}

pub fn check_(var: Var, callback_ref: CallbackRef) -> Stmt {
    Stmt::Check(var, callback_ref)
}
