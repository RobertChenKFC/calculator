use std::iter;
use std::slice::Iter;

use crate::expr::{Expr, ValType};
use crate::func::{Func, Var};
use crate::prog::Prog;
use crate::stmt::Stmt;

pub struct Reference {
    callbacks: Vec<Box<Callback>>,
}

struct VarTable {
    table: Vec<ValType>,
}

impl VarTable {
    fn new<I>(func: &Func, args: I) -> VarTable
    where
        I: Iterator<Item = ValType>,
    {
        let mut table = VarTable { table: vec![] };
        table.table.resize(func.get_num_vars(), 0);
        for (i, arg) in args.enumerate() {
            table.table[i] = arg;
        }
        table
    }

    fn set_var(&mut self, var: Var, val: ValType) {
        self.table[var.0] = val;
    }

    fn get_var(&self, var: Var) -> ValType {
        self.table[var.0]
    }
}

pub type Callback = dyn FnMut(ValType);

#[derive(Clone, Copy)]
pub struct CallbackRef(usize);

impl Reference {
    pub fn new() -> Reference {
        Reference { callbacks: vec![] }
    }

    pub fn register_callback<F: FnMut(ValType) + 'static>(
        &mut self,
        callback: F,
    ) -> CallbackRef {
        let callback_ref = CallbackRef(self.callbacks.len());
        self.callbacks.push(Box::new(callback));
        callback_ref
    }

    pub fn run(&mut self, prog: &Prog) {
        let main_func_ref = prog.get_main_func_ref();
        let main_func = prog.get_func(main_func_ref);
        self.eval_call(prog, main_func, iter::empty::<ValType>());
    }

    fn eval_call<I>(
        &mut self,
        prog: &Prog,
        func: &Func,
        args: I,
    ) -> Option<ValType>
    where
        I: Iterator<Item = ValType>,
    {
        let mut table = VarTable::new(func, args);
        self.eval_body(prog, func.iter(), &mut table)
    }

    fn eval_body(
        &mut self,
        prog: &Prog,
        body: Iter<Stmt>,
        table: &mut VarTable,
    ) -> Option<ValType> {
        for stmt in body {
            if let Some(ret_val) = self.eval_stmt(prog, stmt, table) {
                return Some(ret_val);
            }
        }
        None
    }

    fn eval_stmt(
        &mut self,
        prog: &Prog,
        stmt: &Stmt,
        table: &mut VarTable,
    ) -> Option<ValType> {
        match stmt {
            Stmt::Assign(var, expr) => {
                let val = self.eval_expr(prog, expr, table).unwrap();
                table.set_var(*var, val);
                None
            }
            Stmt::AssignArr(var, idx, expr) => {
                // TODO: this is not correct because this will end up always
                // assigning to the current function's local array, even though
                // the array isn't necessarily from the current function
                let idx: usize = self.eval_expr(prog, idx, table).unwrap().try_into().unwrap();
                let val = self.eval_expr(prog, expr, table).unwrap();
                table.set_var(Var(var.0 + idx), val);
                None
            }
            Stmt::If(cond, body) => {
                let cond_val = self.eval_expr(prog, cond, table).unwrap();
                if cond_val != 0 {
                    self.eval_body(prog, body.iter(), table)
                } else {
                    None
                }
            }
            Stmt::While(cond, body) => {
                while self.eval_expr(prog, cond, table).unwrap() != 0 {
                    if let Some(val) = self.eval_body(prog, body.iter(), table)
                    {
                        return Some(val);
                    }
                }
                None
            }
            Stmt::Do(expr) => {
                self.eval_expr(prog, expr, table);
                None
            }
            Stmt::Debug(message, var) => {
                let val = table.get_var(*var);
                println!("{}: {}", message, val);
                None
            }
            Stmt::Return(expr) => {
                let val = self.eval_expr(prog, expr, table).unwrap();
                Some(val)
            }
            Stmt::Check(var, callback_ref) => {
                let val = table.get_var(*var);
                self.callbacks[callback_ref.0](val);
                None
            }
        }
    }

    fn eval_binary_expr<F: Fn(ValType, ValType) -> ValType>(
        &mut self,
        prog: &Prog,
        table: &mut VarTable,
        lhs: &Expr,
        rhs: &Expr,
        eval: F,
    ) -> Option<ValType> {
        let lhs = self.eval_expr(prog, lhs, table).unwrap();
        let rhs = self.eval_expr(prog, rhs, table).unwrap();
        Some(eval(lhs, rhs))
    }

    fn eval_expr(
        &mut self,
        prog: &Prog,
        expr: &Expr,
        table: &mut VarTable,
    ) -> Option<ValType> {
        match expr {
            Expr::Add(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| {
                    x.wrapping_add(y)
                })
            }
            Expr::Sub(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| {
                    x.wrapping_sub(y)
                })
            }
            Expr::And(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| x & y)
            }
            Expr::Const(val) => Some(*val),
            Expr::Eq(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| {
                    if x == y { 1 } else { 0 }
                })
            }
            Expr::Neq(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| {
                    if x != y { 1 } else { 0 }
                })
            }
            Expr::Lt(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| {
                    if x < y { 1 } else { 0 }
                })
            }
            Expr::Le(lhs, rhs) => {
                self.eval_binary_expr(prog, table, lhs, rhs, |x, y| {
                    if x <= y { 1 } else { 0 }
                })
            }
            Expr::Var(var) => Some(table.get_var(*var)),
            Expr::Call(func_ref, args) => {
                let args: Vec<ValType> = args
                    .iter()
                    .map(|expr| self.eval_expr(prog, expr, table).unwrap())
                    .collect();
                let func = prog.get_func(*func_ref);
                self.eval_call(prog, func, args.into_iter())
            }
            Expr::Index(arr, idx) => {
                // TODO: this is also incorrect due to the same reason as
                // AssignArr above
                let idx: usize = self
                    .eval_expr(prog, idx, table)
                    .unwrap()
                    .try_into()
                    .unwrap();
                Some(table.get_var(Var(arr.0 + idx)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::ToExpr;
    use crate::func::FuncRef;
    use crate::stmt::ToStmt;
    use crate::{body, check_, debug_, if_, let_, return_, while_};

    #[test]
    fn test_fib() {
        let mut reference = Reference::new();
        let mut cnt = 0;
        let num_iters = 10;
        let fib_checker = reference.register_callback(move |val| {
            let result = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];
            assert_eq!(val, result[cnt]);
            cnt += 1;
        });
        let num_iters_clone = num_iters;
        let idx_checker = reference.register_callback(move |val| {
            assert_eq!(val, num_iters_clone);
        });

        let mut prog = Prog::new();
        let fib_ref = prog.register_new_func();
        let fib = prog.get_func_mut(fib_ref);
        let n = fib.get_new_param_var();
        #[rustfmt::skip]
        body!(fib => {
            if_!(n.eq(0) => {
                return_(0);
            });
            if_!(n.eq(1) => {
                return_(1);
            });
            return_(fib_ref.call([n - 1]) + fib_ref.call([n - 2]));
        });

        let main_ref = prog.get_main_func_ref();
        let main = prog.get_func_mut(main_ref);
        let i = main.get_new_local();
        let result = main.get_new_local();
        #[rustfmt::skip]
        body!(main => {
            let_(i, 0);
            while_!(i.lt(num_iters) => {
                let_(result, fib_ref.call([i]));
                check_(result, fib_checker);
                let_(i, i + 1);
            });
            check_(i, idx_checker);
        });

        reference.run(&prog);
    }

    fn check_val(
        reference: &mut Reference,
        var: Var,
        expected_val: ValType,
    ) -> Stmt {
        let checker = reference.register_callback(move |val| {
            assert_eq!(val, expected_val);
        });
        check_(var, checker)
    }

    fn register_div2_func(prog: &mut Prog) -> FuncRef {
        let div2_ref = prog.register_new_func();
        let div2 = prog.get_func_mut(div2_ref);
        let x = div2.get_new_param_var();
        let x_mask = div2.get_new_local();
        let result = div2.get_new_local();
        let result_mask = div2.get_new_local();
        #[rustfmt::skip]
        body!(div2 => {
            let_(result, 0);
            let_(result_mask, 1);
            let_(x_mask, 2);
            while_!(x_mask.neq(0) => {
                if_!(x & x_mask => {
                    let_(result, result + result_mask);
                });
                let_(result_mask, result_mask + result_mask);
                let_(x_mask, x_mask + x_mask);
            });
            if_!(x & result_mask => {
                let_(result, result + result_mask);
            });
            return_(result);
        });
        div2_ref
    }

    #[test]
    fn test_div2() {
        let mut reference = Reference::new();

        let mut prog = Prog::new();
        let div2_ref = register_div2_func(&mut prog);
        let main_ref = prog.get_main_func_ref();
        let main = prog.get_func_mut(main_ref);
        let x = main.get_new_local();
        let y = main.get_new_local();
        #[rustfmt::skip]
        body!(main => {
            let_(x, 5);
            let_(y, div2_ref.call([x]));
            check_val(&mut reference, y, 2);
            let_(x, 38);
            let_(y, div2_ref.call([x]));
            check_val(&mut reference, y, 19);
            let_(x, -119);
            let_(y, div2_ref.call([x]));
            check_val(&mut reference, y, -60);
        });

        reference.run(&prog);
    }

    #[test]
    fn test_merge_sort() {
        let mut reference = Reference::new();

        let mut prog = Prog::new();
        let div2_ref = register_div2_func(&mut prog);
        let merge_sort_ref = prog.register_new_func();
        let merge_sort = prog.get_func_mut(merge_sort_ref);
        let arr = merge_sort.get_new_param_var();
        let from = merge_sort.get_new_param_var();
        let to = merge_sort.get_new_param_var();
        let mid = merge_sort.get_new_local();
        let num_elems = 15;
        let buf = merge_sort.get_new_local_arr(num_elems);
        let i = merge_sort.get_new_local();
        let j = merge_sort.get_new_local();
        let k = merge_sort.get_new_local();
        let x = merge_sort.get_new_local();
        let y = merge_sort.get_new_local();
        body!(merge_sort => {
            // DEBUG
            debug_("From", from);
            debug_("To", to);

            if_!((from + 1).eq(to) => {
                return_(0);
            });
            let_(mid, div2_ref.call([from + to]));
            merge_sort_ref.call([arr, from, mid]);
            merge_sort_ref.call([arr, mid, to]);
            let_(i, from);
            let_(j, mid);
            let_(k, 0);
            while_!(i.lt(mid) & j.lt(to) => {
                let_(x, arr.at(i));
                let_(y, arr.at(j));

                // DEBUG
                debug_("i", i);
                debug_("x", x);
                debug_("j", j);
                debug_("y", y);

                if_!(x.lt(y) => {
                    let_(buf.at(k), x);
                    let_(i, i + 1);
                });
                if_!(y.le(x) => {
                    let_(buf.at(k), y);
                    let_(j, j + 1);
                });
                let_(k, k + 1);
            });
            while_!(i.lt(mid) => {
                // DEBUG
                debug_("Just i", i);

                let_(buf.at(k), arr.at(i));
                let_(i, i + 1);
            });
            while_!(j.lt(to) => {
                // DEBUG
                debug_("Just j", j);

                let_(buf.at(k), arr.at(j));
                let_(j, j + 1);
            });
            let_(i, from);
            let_(k, 0);
            while_!(i.lt(to) => {
                let_(arr.at(i), buf.at(k));
                let_(i, i + 1);
                let_(k, k + 1);
            });
            return_(0);
        });
        let main_ref = prog.get_main_func_ref();
        let main = prog.get_func_mut(main_ref);
        let arr = main.get_new_local_arr(num_elems);
        let i = main.get_new_local();
        let result = main.get_new_local();
        body!(main => {
            let_(arr.at(0), 10);
            let_(arr.at(1), 1);
            let_(arr.at(2), 13);
            let_(arr.at(3), 0);
            let_(arr.at(4), 7);
            let_(arr.at(5), 4);
            let_(arr.at(6), 11);
            let_(arr.at(7), 12);
            let_(arr.at(8), 3);
            let_(arr.at(9), 14);
            let_(arr.at(10), 6);
            let_(arr.at(11), 2);
            let_(arr.at(12), 8);
            let_(arr.at(13), 5);
            let_(arr.at(14), 9);
            merge_sort_ref.call([arr.to_expr(), 0i8.to_expr(), (num_elems as i8 - 1).to_expr()]);
            let_(i, 0);
            while_!(i.lt(num_elems as i8) => {
                // DEBUG
                debug_("Index", i);
                let_(result, arr.at(i));
                debug_("Array elem", result);

                // DEBUG
                // let_(result, arr.at(i).to_expr() - i);
                // check_val(&mut reference, result, 0);
                let_(i, i + 1);
            });
            check_val(&mut reference, i, num_elems as i8);
        });

        reference.run(&prog);
    }
}
