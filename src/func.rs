use std::ops::{Add, BitAnd, Sub};
use std::slice::Iter;

use crate::expr::{Expr, ToExpr};
use crate::stmt::{Stmt, ToAssignStmt};

#[derive(Copy, Clone)]
pub struct Var(pub usize);

impl<Rhs: ToExpr> Add<Rhs> for Var {
    type Output = Expr;
    fn add(self, rhs: Rhs) -> Self::Output {
        Expr::Add(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
}

impl<Rhs: ToExpr> Sub<Rhs> for Var {
    type Output = Expr;
    fn sub(self, rhs: Rhs) -> Self::Output {
        Expr::Sub(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
}

impl<Rhs: ToExpr> BitAnd<Rhs> for Var {
    type Output = Expr;
    fn bitand(self, rhs: Rhs) -> Self::Output {
        Expr::And(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
}

pub struct ArrElem {
    arr: Arr,
    idx: Expr,
}

impl ToExpr for ArrElem {
    fn to_expr(self) -> Expr {
        Expr::Index(self.arr, Box::new(self.idx))
    }
}

impl ToAssignStmt for ArrElem {
    fn to_assign_stmt(self, rhs: Expr) -> Stmt {
        Stmt::AssignArr(self.arr, self.idx, rhs)
    }
}

impl Var {
    pub fn eq<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Eq(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
    pub fn neq<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Neq(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
    pub fn lt<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Lt(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
    pub fn le<Rhs: ToExpr>(self, rhs: Rhs) -> Expr {
        Expr::Le(Box::new(Expr::Var(self)), Box::new(rhs.to_expr()))
    }
}

impl ToExpr for Var {
    fn to_expr(self) -> Expr {
        Expr::Var(self)
    }
}

#[derive(Clone, Copy)]
pub struct Arr(pub usize);

pub const NUM_VARS_PER_ARR_PTR: usize = 2;

impl Arr {
    pub fn at<Idx: ToExpr>(self, idx: Idx) -> ArrElem {
        ArrElem {
            arr: self,
            idx: idx.to_expr(),
        }
    }

    pub fn to_vars(self) -> Vec<Var> {
        let vars = [Var(self.0), Var(self.0 + 1)];
        assert_eq!(vars.len(), NUM_VARS_PER_ARR_PTR);
        vars.into_iter().collect()
    }
}

pub struct Func {
    num_locals: usize,
    local_arrs: Vec<Arr>,
    num_params: usize,
    body: Vec<Stmt>,
    func_ref: FuncRef,
}

#[derive(Clone, Copy)]
pub struct FuncRef(pub usize);

impl Func {
    pub fn new(func_ref: FuncRef) -> Func {
        Func {
            num_locals: 0,
            local_arrs: vec![],
            num_params: 0,
            body: vec![],
            func_ref,
        }
    }

    pub fn get_num_vars(&self) -> usize {
        self.num_params + self.num_locals
    }

    fn get_new_var(&self) -> Var {
        Var(self.get_num_vars())
    }

    fn check_no_locals_declared(&self) {
        assert_eq!(self.num_locals, 0);
    }

    pub fn get_new_param_var(&mut self) -> Var {
        self.check_no_locals_declared();
        let var = self.get_new_var();
        self.num_params += 1;
        var
    }

    pub fn get_new_local_var(&mut self) -> Var {
        let var = self.get_new_var();
        self.num_locals += 1;
        var
    }

    fn get_new_arr(&mut self) -> Arr {
        Arr(self.get_num_vars())
    }

    pub fn get_new_param_arr(&mut self) -> Arr {
        self.check_no_locals_declared();
        let arr = self.get_new_arr();
        self.num_params += NUM_VARS_PER_ARR_PTR;
        arr
    }

    pub fn get_new_local_arr(&mut self, len: usize) -> Arr {
        let arr = self.get_new_arr();
        self.local_arrs.push(arr);
        self.num_locals += NUM_VARS_PER_ARR_PTR + len;
        arr
    }

    pub fn get_local_arrs(&self) -> Iter<Arr> {
        self.local_arrs.iter()
    }

    pub fn set_body<const N: usize>(&mut self, body: [Stmt; N]) {
        self.body = body.into_iter().collect();
    }

    pub fn get_ref(&self) -> FuncRef {
        self.func_ref
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, Stmt> {
        self.body.iter()
    }
}

pub enum Arg {
    Expr(Expr),
    Arr(Arr),
}

impl Arg {
    fn to_exprs(self) -> Vec<Expr> {
        match self {
            Arg::Expr(expr) => vec![expr],
            Arg::Arr(arr) => {
                arr.to_vars().into_iter().map(|var| var.to_expr()).collect()
            }
        }
    }
}

pub trait ToArg {
    fn to_arg(self) -> Arg;
}

impl<T: ToExpr> ToArg for T {
    fn to_arg(self) -> Arg {
        Arg::Expr(self.to_expr())
    }
}

impl ToArg for Arr {
    fn to_arg(self) -> Arg {
        Arg::Arr(self)
    }
}

impl FuncRef {
    pub fn call<const N: usize>(self, args: [Arg; N]) -> Expr {
        let mut exprs: Vec<Expr> = vec![];
        for arg in args {
            exprs.append(&mut arg.to_exprs());
        }
        Expr::Call(self, exprs)
    }
}

#[macro_export]
macro_rules! call {
    ($func:ident($($arg:expr),*$(,)?)) => {
        $func.call([$($arg.to_arg()),*])
    }
}

#[macro_export]
macro_rules! body{
    ($func:expr => {$($stmt:expr);*$(;)?}) => {
        ($func).set_body([
            $(($stmt).to_stmt()),*
        ])
    }
}
