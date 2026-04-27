use crate::expr::{Expr, ToExpr, ValType};
use crate::func::{Arr, Func, Var};
use crate::prog::Prog;
use crate::reference::CallbackRef;

pub struct CondBody {
    pub cond: Expr,
    pub body: Vec<Stmt>,
}
pub enum Stmt {
    If(Vec<CondBody>, Vec<Stmt>),
    Return(Expr),
    Assign(Var, Expr),
    AssignArr(Arr, Expr, Expr),
    Do(Expr),
    While(Expr, Vec<Stmt>),
    SetOutput(Expr, Expr),
    ShowOutput,
    CheckOutput(&'static str),
    Debug(&'static str, Vec<Expr>),
    Check(Var, CallbackRef),
}

impl Stmt {
    fn validate_body(body: &Vec<Stmt>, prog: &Prog, func: &Func) {
        for stmt in body {
            stmt.validate(prog, func);
        }
    }

    pub fn validate(&self, prog: &Prog, func: &Func) {
        match self {
            Stmt::If(cond_bodies, else_body) => {
                for cond_body in cond_bodies {
                    cond_body.cond.validate(prog, func, self);
                    Stmt::validate_body(&cond_body.body, prog, func);
                }
                Stmt::validate_body(else_body, prog, func);
            }
            Stmt::Return(expr) => {
                expr.validate(prog, func, self);
            }
            Stmt::Assign(var, expr) => {
                expr.validate(prog, func, self);
            }
            Stmt::AssignArr(var, idx, val) => {
                idx.validate(prog, func, self);
                val.validate(prog, func, self);
            }
            Stmt::Do(expr) => {
                expr.validate(prog, func, self);
            }
            Stmt::While(expr, body) => {
                expr.validate(prog, func, self);
                Stmt::validate_body(body, prog, func);
            }
            Stmt::SetOutput(idx, val) => {
                idx.validate(prog, func, self);
                val.validate(prog, func, self);
            }
            Stmt::ShowOutput => {}
            Stmt::CheckOutput(msg) => {}
            Stmt::Debug(msg, exprs) => {
                for expr in exprs {
                    expr.validate(prog, func, self);
                }
            }
            Stmt::Check(val, callback) => {}
        }
    }
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
    (
        $if_cond:expr => {$($if_stmt:expr);*$(;)?}
        $(else if $elif_cond:expr => {$($elif_stmt:expr);*$(;)?})*
        $(else => {$($else_stmt:expr);*$(;)?})?
    ) =>
    { Stmt::If(
        vec![
            CondBody { cond: ($if_cond).to_expr(), body: vec![$($if_stmt),*] },
            $(CondBody {
                cond: ($elif_cond).to_expr(),
                body: vec![$($elif_stmt),*]
            }),*],
        vec![$($($else_stmt),*)?]) }
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
    ($cond:expr => {$($stmt:expr);*$(;)?}) => {
        Stmt::While(($cond).to_expr(), vec![$(($stmt).to_stmt()),*])
    }
}

#[macro_export]
macro_rules! debug_ {
    ($message:expr$(, $($exprs:expr),*$(,)?)?) => {
        Stmt::Debug(($message), vec![$($(($exprs).to_expr()),*)?])
    }
}

pub fn check_(var: Var, callback_ref: CallbackRef) -> Stmt {
    Stmt::Check(var, callback_ref)
}

pub fn set_output_<T: ToExpr, U: ToExpr>(index: T, value: U) -> Stmt {
    Stmt::SetOutput(index.to_expr(), value.to_expr())
}

pub fn show_output_() -> Stmt {
    Stmt::ShowOutput
}

pub fn check_output_(output: &'static str) -> Stmt {
    Stmt::CheckOutput(output)
}
