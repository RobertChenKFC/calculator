use std::iter;
use std::slice::Iter;

use crate::expr::{Expr, ValType};
use crate::func::{Arr, Func, NUM_VARS_PER_ARR_PTR, Var, ToArg};
use crate::prog::Prog;
use crate::stmt::Stmt;

pub struct Reference {
    callbacks: Vec<Box<Callback>>,
    stack: Stack,
}

struct Stack {
    stack: Vec<ValType>,
    frame_pointers: Vec<usize>,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack: vec![],
            frame_pointers: vec![]
        }
    }

    fn push_frame<I>(&mut self, func: &Func, args: I)
    where I: Iterator<Item = ValType> {
        let frame_pointer = self.stack.len();
        self.frame_pointers.push(frame_pointer);
        self.stack.resize(self.stack.len() + func.get_num_vars(), 0);
        for (i, arg) in args.enumerate() {
            self.stack[frame_pointer + i] = arg;
        }
        for arr in func.get_local_arrs() {
            self.set_arr_pointer(arr, frame_pointer);
        }
    }

    fn pop_frame(&mut self, func: &Func) {
        for _ in 0..func.get_num_vars() {
            self.stack.pop();
        }
        self.frame_pointers.pop();
    }

    fn set_var(&mut self, var: Var, val: ValType) {
        let frame_pointer = self.frame_pointers.last().unwrap();
        self.stack[frame_pointer + var.0] = val;
    }

    fn get_var(&self, var: Var) -> ValType {
        let frame_pointer = self.frame_pointers.last().unwrap();
        self.stack[frame_pointer + var.0]
    }

    fn set_arr_pointer(&mut self, arr: &Arr, frame_pointer: usize) {
        let arr_vars = arr.to_vars();
        let arr_pointer = frame_pointer + arr_vars.first().unwrap().0 + NUM_VARS_PER_ARR_PTR;
        for (byte, var) in arr_pointer.to_le_bytes().into_iter().zip(arr_vars) {
            self.set_var(var, byte as ValType);
        }
    }

    fn get_arr_pointer(&self, arr: &Arr) -> usize {
        let mut arr_pointer = [0u8; size_of::<usize>()];
        assert!(NUM_VARS_PER_ARR_PTR < arr_pointer.len());
        for (byte, var) in arr_pointer.iter_mut().zip(arr.to_vars()) {
            *byte = self.get_var(var) as u8;
        }
        let arr_pointer = usize::from_le_bytes(arr_pointer);
        arr_pointer
    }

    fn set_arr(&mut self, arr: &Arr, idx: ValType, val: ValType) {
        let idx = self.get_arr_pointer(arr) + idx as usize;
        self.stack[idx] = val;
    }

    fn get_arr(&self, arr: &Arr, idx: ValType) -> ValType {
        let idx = self.get_arr_pointer(arr) + idx as usize;
        self.stack[idx]
    }
}

pub type Callback = dyn FnMut(ValType);

#[derive(Clone, Copy)]
pub struct CallbackRef(usize);

impl Reference {
    pub fn new() -> Reference {
        Reference { callbacks: vec![], stack: Stack::new() }
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
        self.stack.push_frame(func, args);
        let val = self.eval_body(prog, func.iter());
        self.stack.pop_frame(func);
        val
    }

    fn eval_body(
        &mut self,
        prog: &Prog,
        body: Iter<Stmt>,
    ) -> Option<ValType> {
        for stmt in body {
            if let Some(ret_val) = self.eval_stmt(prog, stmt) {
                return Some(ret_val);
            }
        }
        None
    }

    fn eval_stmt(
        &mut self,
        prog: &Prog,
        stmt: &Stmt,
    ) -> Option<ValType> {
        match stmt {
            Stmt::Assign(var, expr) => {
                let val = self.eval_expr(prog, expr).unwrap();
                self.stack.set_var(*var, val);
                None
            }
            Stmt::AssignArr(arr, idx, expr) => {
                let idx = self.eval_expr(prog, idx).unwrap();
                let val = self.eval_expr(prog, expr).unwrap();
                self.stack.set_arr(arr, idx, val);
                None
            }
            Stmt::If(cond, body) => {
                let cond_val = self.eval_expr(prog, cond).unwrap();
                if cond_val != 0 {
                    self.eval_body(prog, body.iter())
                } else {
                    None
                }
            }
            Stmt::While(cond, body) => {
                while self.eval_expr(prog, cond).unwrap() != 0 {
                    if let Some(val) = self.eval_body(prog, body.iter())
                    {
                        return Some(val);
                    }
                }
                None
            }
            Stmt::Do(expr) => {
                self.eval_expr(prog, expr);
                None
            }
            Stmt::Debug(message, exprs) => {
                // TODO: do a more advanced format string parsing here.
                // Unfortunately, rust only accepts static string formatting
                let mut match_right_brace = false;
                let mut i = 0;
                for c in message.chars() {
                    assert_eq!(match_right_brace, c == '}');
                    match c {
                        '{' => match_right_brace = true,
                        '}' => {
                            match_right_brace = false;
                            let val = self.eval_expr(prog, &exprs[i]).unwrap();
                            print!("{}", val);
                            i += 1;
                        },
                        _ => {
                            match_right_brace = false;
                            print!("{}", c);
                        }
                    }
                }
                println!();
                None
            }
            Stmt::Return(expr) => {
                let val = self.eval_expr(prog, expr).unwrap();
                Some(val)
            }
            Stmt::Check(var, callback_ref) => {
                let val = self.stack.get_var(*var);
                self.callbacks[callback_ref.0](val);
                None
            }
        }
    }

    fn eval_binary_expr<F: Fn(ValType, ValType) -> ValType>(
        &mut self,
        prog: &Prog,
        lhs: &Expr,
        rhs: &Expr,
        eval: F,
    ) -> Option<ValType> {
        let lhs = self.eval_expr(prog, lhs).unwrap();
        let rhs = self.eval_expr(prog, rhs).unwrap();
        Some(eval(lhs, rhs))
    }

    fn eval_expr(
        &mut self,
        prog: &Prog,
        expr: &Expr,
    ) -> Option<ValType> {
        match expr {
            Expr::Add(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| {
                    x.wrapping_add(y)
                })
            }
            Expr::Sub(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| {
                    x.wrapping_sub(y)
                })
            }
            Expr::And(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| x & y)
            }
            Expr::Const(val) => Some(*val),
            Expr::Eq(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| {
                    if x == y { 1 } else { 0 }
                })
            }
            Expr::Neq(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| {
                    if x != y { 1 } else { 0 }
                })
            }
            Expr::Lt(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| {
                    if x < y { 1 } else { 0 }
                })
            }
            Expr::Le(lhs, rhs) => {
                self.eval_binary_expr(prog, lhs, rhs, |x, y| {
                    if x <= y { 1 } else { 0 }
                })
            }
            Expr::Var(var) => Some(self.stack.get_var(*var)),
            Expr::Call(func_ref, args) => {
                let args: Vec<ValType> = args
                    .iter()
                    .map(|expr| self.eval_expr(prog, expr).unwrap())
                    .collect();
                let func = prog.get_func(*func_ref);
                self.eval_call(prog, func, args.into_iter())
            }
            Expr::Index(arr, idx) => {
                let idx= self
                    .eval_expr(prog, idx)
                    .unwrap();
                Some(self.stack.get_arr(arr, idx))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::ToExpr;
    use crate::func::{FuncRef, ToArg};
    use crate::stmt::ToStmt;
    use crate::{body, call, check_, debug_, if_, let_, return_, while_};

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
            return_(call!(fib_ref(n - 1)) + call!(fib_ref(n - 2)));
        });

        let main_ref = prog.get_main_func_ref();
        let main = prog.get_func_mut(main_ref);
        let i = main.get_new_local_var();
        let result = main.get_new_local_var();
        #[rustfmt::skip]
        body!(main => {
            let_(i, 0);
            while_!(i.lt(num_iters) => {
                let_(result, call!(fib_ref(i)));
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
        let x_mask = div2.get_new_local_var();
        let result = div2.get_new_local_var();
        let result_mask = div2.get_new_local_var();
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
        let x = main.get_new_local_var();
        let y = main.get_new_local_var();
        #[rustfmt::skip]
        body!(main => {
            let_(x, 5);
            let_(y, call!(div2_ref(x)));
            check_val(&mut reference, y, 2);
            let_(x, 38);
            let_(y, call!(div2_ref(x)));
            check_val(&mut reference, y, 19);
            let_(x, -119);
            let_(y, call!(div2_ref(x)));
            check_val(&mut reference, y, -60);
        });

        reference.run(&prog);
    }

    #[test]
    fn test_arr() {
        let mut reference = Reference::new();

        let mut idx = 0;
        let arr_checker = reference.register_callback(move |val| {
            idx += 1;
            assert_eq!(val, 3 * idx);
        });

        let mut prog = Prog::new();
        let foo_ref = prog.register_new_func();
        let foo = prog.get_func_mut(foo_ref);
        let arr = foo.get_new_param_arr();
        let i = foo.get_new_local_var();
        let x = foo.get_new_local_var();
        let num_elems = 5;
        body!(foo => {
            let_(i, 0);
            while_!(i.lt(num_elems) => {
                let_(x, arr.at(i));
                let_(arr.at(i), x + x + x);
                let_(i, i + 1);
            });
        });
        let main_ref = prog.get_main_func_ref();
        let main = prog.get_func_mut(main_ref);
        let i = main.get_new_local_var();
        let arr = main.get_new_local_arr(num_elems as usize);
        body!(main => {
            let_(i, 0);
            while_!(i.lt(num_elems) => {
                let_(arr.at(i), i + 1);
                let_(i, i + 1);
            });
            call!(foo_ref(arr));
            let_(i, 0);
            while_!(i.lt(num_elems) => {
                let_(x, arr.at(i));
                check_(x, arr_checker);
                let_(i, i + 1);
            });
            check_val(&mut reference, i, num_elems);
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
        let arr = merge_sort.get_new_param_arr();
        let from = merge_sort.get_new_param_var();
        let to = merge_sort.get_new_param_var();
        let mid = merge_sort.get_new_local_var();
        let num_elems = 15;
        let buf = merge_sort.get_new_local_arr(num_elems);
        let i = merge_sort.get_new_local_var();
        let j = merge_sort.get_new_local_var();
        let k = merge_sort.get_new_local_var();
        let x = merge_sort.get_new_local_var();
        let y = merge_sort.get_new_local_var();
        body!(merge_sort => {
            if_!((from + 1).eq(to) => {
                return_(0);
            });
            let_(mid, call!(div2_ref(from + to)));
            call!(merge_sort_ref(arr, from, mid));
            call!(merge_sort_ref(arr, mid, to));
            let_(i, from);
            let_(j, mid);
            let_(k, 0);
            while_!(i.lt(mid) & j.lt(to) => {
                let_(x, arr.at(i));
                let_(y, arr.at(j));
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
                let_(buf.at(k), arr.at(i));
                let_(i, i + 1);
                let_(k, k + 1);
            });
            while_!(j.lt(to) => {
                let_(buf.at(k), arr.at(j));
                let_(j, j + 1);
                let_(k, k + 1);
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
        let i = main.get_new_local_var();
        let result = main.get_new_local_var();
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
            call!(merge_sort_ref(arr, 0i8, (num_elems as i8)));
            let_(i, 0);
            while_!(i.lt(num_elems as i8) => {
                let_(result, arr.at(i));
                let_(result, arr.at(i).to_expr() - i);
                check_val(&mut reference, result, 0);
                let_(i, i + 1);
            });
            check_val(&mut reference, i, num_elems as i8);
        });

        reference.run(&prog);
    }
}
