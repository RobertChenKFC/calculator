use std::ops::{Add, Sub, BitAnd};
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
pub struct Arr {
    arr_idx: usize,
    head_elem: Var,
}

impl Arr {
    pub fn at<Idx: ToExpr>(self, idx: Idx) -> ArrElem {
        ArrElem { arr: self, idx: idx.to_expr() }
    }
}

pub struct Func {
    num_param_vars: usize,
    num_local_vars: usize,
    num_param_arrs: usize,
    num_local_arrs: usize,
    body: Vec<Stmt>,
    func_ref: FuncRef,
}

#[derive(Clone, Copy)]
pub struct FuncRef(pub usize);

impl Func {
    pub fn new(func_ref: FuncRef) -> Func {
        Func {
            num_param_vars: 0,
            num_local_vars: 0,
            num_param_arrs: 0,
            num_local_arrs: 0,
            body: vec![],
            func_ref,
        }
    }

    fn get_num_vars(&self) -> usize {
        self.num_param_vars + self.num_local_vars
    }

    fn get_new_var<F: Fn(&mut Func)>(&mut self, modify_cnt: F) -> Var {
        let var = Var(self.get_num_vars());
        modify_cnt(self);
        var
    }

    fn check_no_locals_declared(&self) {
        assert_eq!(self.num_local_arrs, 0);
        assert_eq!(self.num_local_vars, 0);
    }

    pub fn get_new_param_var(&mut self) -> Var {
        self.check_no_locals_declared();
        self.get_new_var(|func| func.num_param_vars += 1)
    }

    pub fn get_new_local_var(&mut self) -> Var {
        self.get_new_var(|func| func.num_local_vars += 1)
    }

    fn get_num_arrs(&self) -> usize {
        self.num_param_arrs + self.num_local_arrs
    }

    fn get_new_arr<F: Fn(&mut Func)>(&mut self, modify_cnt: F) -> Arr {
        let arr = Arr {
            arr_idx: self.get_num_arrs(),
            head_elem: Var(self.get_num_vars());
        };
        modify_cnt(self);
        arr
    }

    pub fn get_new_param_arr(&mut self) -> Arr {
        self.check_no_locals_declared();
        self.get_new_arr(|func| {
            func.num_param_arrs += 1;
        })
    }

    pub fn get_new_local_arr(&mut self, len: usize) -> Arr {
        self.get_new_arr(|func| {
            func.num_local_arrs += 1;
            func.num_local_vars += len;
        })
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

impl FuncRef {
    pub fn call<T: ToExpr, const N: usize>(self, args: [T; N]) -> Expr {
        let args = args.into_iter().map(|x| x.to_expr()).collect();
        Expr::Call(self, args)
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
