use crate::expr::{FALSE, TRUE, ToExpr, ValType};
use crate::func::{Arr, Func, FuncRef, ToArg};
use crate::prog::Prog;
use crate::seven_segment::{DIGITS, NUM_DIGITS, RADIX, SEG_DECIMAL, SEG_MINUS};
use crate::stmt::{CondBody, Stmt, ToStmt, set_output_, show_output_};
use crate::{append_body, body, call, debug_, if_, let_, return_, while_};

const ERR_OVERFLOW: ValType = 1;

struct Calc {
    // The number is stored as an array of length `2*num_digits+1`, where the
    // elements of the array represent the following:
    //
    //   0   | 1 ... num_digits | num_digits+1 ... 2*num_digits
    //  sign |   integer part   |        decimal part
    num_digits: ValType,
}

struct MainFunc {
    main_ref: FuncRef,
    digits: Arr,
}

impl Calc {
    fn new(num_digits: ValType) -> Calc {
        Calc { num_digits }
    }

    fn num_elems(&self) -> ValType {
        self.num_digits * 2 + 1
    }

    fn sign_idx(&self) -> ValType {
        0
    }

    fn integer_part_start_idx(&self) -> ValType {
        1
    }

    fn decimal_part_start_idx(&self) -> ValType {
        self.num_digits + 1
    }

    fn register_init_func(&self, prog: &mut Prog) -> FuncRef {
        let init_ref = prog.register_new_func();
        let init = prog.get_func_mut(init_ref);

        let digits = init.get_new_param_arr();
        init.set_body(
            (0..RADIX)
                .into_iter()
                .map(|x| let_(digits.at(x as ValType), DIGITS[x]))
                .collect(),
        );

        init_ref
    }

    fn init_main_func(&self, prog: &mut Prog) -> MainFunc {
        let init_ref = self.register_init_func(prog);
        let main_ref = prog.get_main_func_ref();
        let main = prog.get_func_mut(main_ref);

        let digits = main.get_new_local_arr(RADIX);
        body!(main => {
            call!(init_ref(digits));
        });
        MainFunc { main_ref, digits }
    }

    fn register_display_func(&self, prog: &mut Prog) -> FuncRef {
        let display_ref = prog.register_new_func();
        let display = prog.get_func_mut(display_ref);

        // The display is of fixed width `NUM_DIGITS`, while the calculator
        // number has `self.num_digits` both before and after the decimal point.
        // Therefore, we try to display as many digits as possible, and report
        // an error if we cannot display the number without losing digits before
        // the decimal point.
        let digits = display.get_new_param_arr();
        let num = display.get_new_param_arr();
        let i = display.get_new_local_var();
        let j = display.get_new_local_var();
        let len = display.get_new_local_var();
        let sign_len = display.get_new_local_var();
        let display_idx = display.get_new_local_var();
        let digit = display.get_new_local_var();
        body!(display => {
            if_!(num.at(self.sign_idx()) => {
                let_(sign_len, 1);
            } else => {
                let_(sign_len, 0);
            });

            let_(i, self.integer_part_start_idx());
            // Point `i` to the first non-zero index on the integer part. If
            // the integer part is 0, this will point to the last digit of the
            // integer part. This is because we always want to display at least
            // one digit from the integer part, otherwise we might miss the
            // decimal point because on the 7-segment display, a decimal point
            // must follow a digit.
            while_!(i.lt(self.decimal_part_start_idx() - 1) &
                    num.at(i).eq(0) => {
                let_(i, i + 1);
            });

            // The integer part starts from `i` and ends at
            // `self.decimal_part_start_idx()-1`, so we derive the integer
            // length from this. If the number is negative, we have to increase
            // the length by 1.
            let_(len, self.decimal_part_start_idx().to_expr() - i);
            if_!((len + sign_len).gt(NUM_DIGITS as ValType) => {
                // The integer part does not fit in the display, so we return an
                // error.
                return_(ERR_OVERFLOW);
            });

            // Point `j` to the last non-zero index.
            let_(j, self.num_elems() - 1);
            while_!(j.gt(i) & num.at(j).eq(0) => {
                let_(j, j - 1);
            });

            // Now, we make `len` to hold the number of digits (including the
            // minus sign) we want to display.
            let_(len, j - i + 1 + sign_len);
            // If `len` is longer than the display length, we need to truncate
            // the later digits. The check at the start guaranteed that we will
            // not lose digits before the decimal points if we do so.
            if_!(len.gt(NUM_DIGITS as ValType) => {
                let_(len, NUM_DIGITS as ValType);
                let_(j, i + (len - sign_len) - 1);
            });
            // Note that after truncation, it's possible all the non-zero
            // decimal digits got truncated, and now we're left with all zeros
            // after the decimal to display. If that's the case, truncate even
            // further so that we are left with just the integer part.
            while_!(j.ge(self.decimal_part_start_idx()) & num.at(j).eq(0) => {
                let_(j, j - 1);
            });
            let_(len, j - i + 1 + sign_len);

            // It's possible that `len` is shorter than the display length. In
            // this case, we want the digits to be right justified:
            //    display_idx + len = NUM_DIGITS
            // => display_idx = NUM_DIGITS - len
            let_(display_idx, 0);
            while_!(display_idx.lt((NUM_DIGITS as ValType).to_expr() - len) => {
                set_output_(display_idx, 0);
                let_(display_idx, display_idx + 1);
            });

            // Display the minus sign first.
            if_!(sign_len.gt(0) => {
                set_output_(display_idx, 1 << SEG_MINUS);
                let_(display_idx, display_idx + 1);
            });

            // Then, display the rest of the digits.
            while_!(display_idx.lt(NUM_DIGITS as ValType) => {
                let_(digit, digits.at(num.at(i)));
                // The `self.decimal_part_start_idx() - 1`-th digit is the last
                // digit of the integer part. Therefore, we should add a decimal
                // point here. However, if there are no more digits after this,
                // then this means the result is an integer, so we don't add a
                // decimal point in this case.
                if_!(i.eq(self.decimal_part_start_idx() - 1) & i.neq(j) => {
                    let_(digit, digit | ((1 << SEG_DECIMAL) as ValType));
                });
                set_output_(display_idx, digit);
                let_(i, i + 1);
                let_(display_idx, display_idx + 1);
            });
            return_(0);
        });

        display_ref
    }

    fn register_add_func(&self, prog: &mut Prog) -> FuncRef {
        let add_ref = prog.register_new_func();
        let add = prog.get_func_mut(add_ref);
        let num1 = add.get_new_param_arr();
        let num2 = add.get_new_param_arr();
        let is_sub = add.get_new_param_var();
        let i = add.get_new_local_var();
        let x = add.get_new_local_var();
        let carry = add.get_new_local_var();
        let num1_sign = add.get_new_local_var();
        let num2_sign = add.get_new_local_var();
        let ret = add.get_new_local_var();
        body!(add => {
            let_(num1_sign, num1.at(self.sign_idx()));
            let_(num2_sign, num2.at(self.sign_idx()));
            // We have (+/- num1) +/- (+/- num2)
            //                    (1)  (2)
            // In this step, we are combining (1) and (2) together into
            // `is_sub`, and clearing out the sign element of `num2`.
            if_!(num2_sign => {
                let_(is_sub, !is_sub);
                let_(num2.at(self.sign_idx()), FALSE);
            });
            // Now, we have (+/- num1) (+/- num2).
            if_!(num1_sign => {
                // - If `is_sub` is true, then we are computing -num1 - num2,
                //   which is equivalent to -(num2 + num1).
                // - If `is_sub` is false, then we are computing -num1 + num2,
                //   which is equivlanet to num2 - num1.
                // Therefore, we compute with a flipped `is_sub`, and flip back
                // according to `is_sub`. Either way, we clear the sign element
                // of `num1`.
                let_(num1.at(self.sign_idx()), FALSE);
                let_(ret, call!(add_ref(num2, num1, !is_sub)));
                // Because we swapped `num1` and `num2`, and the result is
                // stored in `num1`, we have to copy over the results from
                // `num2`.
                let_(i, 0);
                while_!(i.lt(self.num_elems()) => {
                    let_(num1.at(i), num2.at(i));
                    let_(i, i + 1);
                });
                // Then flip the sign back.
                if_!(is_sub => {
                    let_(num1.at(self.sign_idx()),
                         !num1.at(self.sign_idx()).to_expr());
                });
                return_(ret);
            });

            // If we get here, that means both `num1` and `num2` are >= 0, and
            // we can proceed with addition/subtraction ignoring the signs.
            let_(i, self.num_elems() - 1);
            if_!(is_sub => {
                let_(carry, 1);
            } else => {
                let_(carry, 0);
            });
            while_!(i.ge(self.integer_part_start_idx()) => {
                let_(x, carry + num1.at(i));
                if_!(is_sub => {
                    // Subtraction is performed using 10's complement.
                    let_(x, x + 9 - num2.at(i));
                } else => {
                    let_(x, x + num2.at(i));
                });
                if_!(x.ge(10) => {
                    let_(x, x - 10);
                    let_(carry, 1);
                } else => {
                    let_(carry, 0);
                });
                let_(num1.at(i), x);
                let_(i, i - 1);
            });
            if_!(!is_sub & carry.eq(1) => {
                // We were performing addition and the carry overflowed.
                return_(ERR_OVERFLOW);
            });
            if_!(is_sub & carry.eq(0) => {
                // We were performing subtraction. Since 10's complement was
                // used, if we don't get an overflow, that means the sign of
                // the result flipped. Therefore, we have to flip it back by
                // performing a 10's complement on the result once again.
                let_(i, self.num_elems() - 1);
                let_(carry, 1);
                while_!(i.ge(self.integer_part_start_idx()) => {
                    let_(x, carry + 9 - num1.at(i));
                    if_!(x.ge(10) => {
                        let_(x, x - 10);
                        let_(carry, 1);
                    } else => {
                        let_(carry, 0);
                    });
                    let_(num1.at(i), x);
                    let_(i, i - 1);
                });
                let_(num1.at(self.sign_idx()), TRUE);
            });
            return_(0);
        });
        add_ref
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::TRUE, numpad::TermNumpad, reference::Reference,
        stmt::check_output_,
    };

    use super::*;

    fn set_num(calc: &Calc, func: &mut Func, num: Arr, val: &str) {
        // First, clear all digits in `num` to 0, and clear the sign element.
        append_body!(func => {
            let_(num.at(calc.sign_idx()), FALSE);
        });
        for i in calc.integer_part_start_idx()..calc.num_elems() {
            append_body!(func => {
                let_(num.at(i), 0);
            });
        }

        // Check if the value starts with a minus sign. If so, set the
        // corresponding sign element. Then, remove the minus sign from the
        // start for subsequent digit extraction.
        let is_neg = val.starts_with("-");
        let val = if is_neg {
            append_body!(func => {
                let_(num.at(calc.sign_idx()), TRUE);
            });
            &val[1..]
        } else {
            val
        };

        let val: Vec<_> = val.chars().collect();
        let mut decimal_idx = 0;
        while decimal_idx < val.len() && val[decimal_idx] != '.' {
            decimal_idx += 1;
        }
        assert!(decimal_idx < calc.num_digits as usize);
        // Handle digits before the decimal.
        for (i, c) in val[..decimal_idx].iter().enumerate() {
            // i         = 0..decimal_idx-1
            // digit_idx = x..decimal_part_start_idx-1
            // => x = decimal_part_start_idx-decimal_idx
            let digit_idx = calc.decimal_part_start_idx()
                - (decimal_idx as ValType)
                + (i as ValType);
            append_body!(func => {
                let_(num.at(digit_idx), c.to_digit(10).unwrap() as ValType);
            });
        }
        if decimal_idx + 1 >= val.len() {
            return;
        }
        // Handle digits after the decimal.
        for (i, c) in val[decimal_idx + 1..].iter().enumerate() {
            let digit_idx = calc.decimal_part_start_idx() + (i as ValType);
            append_body!(func => {
                let_(num.at(digit_idx), c.to_digit(10).unwrap() as ValType);
            });
        }
    }

    #[test]
    fn test_display() {
        let mut reference: Reference<TermNumpad> = Reference::new();

        let calc = Calc::new(NUM_DIGITS as ValType);
        let mut prog = Prog::new();
        let main_func = calc.init_main_func(&mut prog);
        let display_ref = calc.register_display_func(&mut prog);

        let main = prog.get_func_mut(main_func.main_ref);
        let num = main.get_new_local_arr(calc.num_elems() as usize);
        let ret = main.get_new_local_var();
        for (val, reference_val) in [
            ("1963504782", "1963504782"),
            ("3.14", "3.14"),
            ("1274898.978123", "1274898.978123"),
            ("0", "0"),
            ("0.123897120124781", "0.123897120124781"),
            ("-8160274953", "-8160274953"),
            ("-9283604571.27103", "-9283604571.27103"),
            ("1234567890.0000001", "1234567890"),
        ] {
            set_num(&calc, main, num, val);
            append_body!(main => {
                let_(ret, call!(display_ref(main_func.digits, num)));
                reference.check_val(ret, 0);
                check_output_(reference_val);
            });
        }

        reference.run(&prog);
    }

    #[test]
    fn test_add() {
        let mut reference: Reference<TermNumpad> = Reference::new();

        let calc = Calc::new(NUM_DIGITS as ValType);
        let mut prog = Prog::new();
        let main_func = calc.init_main_func(&mut prog);
        let display_ref = calc.register_display_func(&mut prog);
        let add_ref = calc.register_add_func(&mut prog);

        let main = prog.get_func_mut(main_func.main_ref);
        let num1 = main.get_new_local_arr(calc.num_elems() as usize);
        let num2 = main.get_new_local_arr(calc.num_elems() as usize);
        let ret = main.get_new_local_var();
        for (val1, val2, result) in [
            ("1963504782", "1348709265", "3312214047"),
            ("900.007418129072", "77.6896211931742", "977.6970393222462"),
            ("0.9999999999999999", "0.0000000000000001", "1"),
            (
                "102755.7127917481",
                "-30.80079466390691",
                "102724.9119970841",
            ),
            (
                "-76071446731332.01",
                "6837960.126995179",
                "-76071439893371.8",
            ),
            (
                "-286.6725402302072",
                "-0.222565620154703",
                "-286.895105850361",
            ),
        ] {
            set_num(&calc, main, num1, val1);
            set_num(&calc, main, num2, val2);
            append_body!(main => {
                let_(ret, call!(add_ref(num1, num2, /*is_sub=*/FALSE)));
                reference.check_val(ret, 0);
                let_(ret, call!(display_ref(main_func.digits, num1)));
                reference.check_val(ret, 0);
                check_output_(result);
            });
        }

        reference.run(&prog);
    }
}
