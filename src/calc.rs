use crate::expr::{ToExpr, ValType};
use crate::func::FuncRef;
use crate::prog::Prog;
use crate::stmt::{CondBody, Stmt, ToStmt};
use crate::{body, if_, let_, while_};

struct Calc {
    // The number is stored as a big decimal of `num_digits` both before and
    // after the decimal point (ie. the total number of digits is actually
    // 2 * `num_digits`).
    num_digits: usize,
}

impl Calc {
    fn new(num_digits: usize) -> Calc {
        Calc { num_digits }
    }

    fn register_display_func(&self, prog: &mut Prog) -> FuncRef {
        let display_ref = prog.register_new_func();
        let display = prog.get_func_mut(display_ref);

        // The display is of fixed width `NUM_DIGITS`, while the calculator
        // number has `self.num_digits` both before and after the decimal point.
        // Therefore, we try to display as many digits as possible, and report
        // an error if we cannot display the number without losing digits before
        // the decimal point.
        let num = display.get_new_param_arr();
        let i = display.get_new_local_var();
        let num_integer_digits = display.get_new_local_var();
        body!(display => {
            let_(i, 0);
            let_(num_integer_digits, self.num_digits as ValType);
            while_!(i.lt(self.num_digits as ValType) & num.at(i).eq(0) => {
                let_(num_integer_digits, num_integer_digits - 1);
                let_(i, i + 1);
            });
        });

        display_ref
    }

    fn register_add_func(&self, prog: &mut Prog) -> FuncRef {
        let add_ref = prog.register_new_func();
        let add = prog.get_func_mut(add_ref);
        let num1 = add.get_new_param_arr();
        let num2 = add.get_new_param_arr();
        let i = add.get_new_local_var();
        let x = add.get_new_local_var();
        let carry = add.get_new_local_var();
        body!(add => {
            let_(i, 0);
            let_(carry, 0);
            while_!(i.lt((self.num_digits * 2) as ValType) => {
                let_(x, carry + num1.at(i) + num2.at(i));
                if_!(x.ge(10) => {
                    let_(x, x - 10);
                    let_(carry, 1);
                });
                let_(num1.at(i), x);
                let_(i, i + 1);
            });
        });
        add_ref
    }
}
