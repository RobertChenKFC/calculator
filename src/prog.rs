use crate::func::{Func, FuncRef};

pub struct Prog {
    funcs: Vec<Func>,
    main_func_ref: FuncRef,
}

impl Prog {
    pub fn new() -> Prog {
        let mut prog = Prog {
            funcs: vec![],
            // This is just a placeholder, even though it happens to be the
            // correct value. It will be replaced by the actually main function
            // index later.
            main_func_ref: FuncRef(0),
        };
        prog.main_func_ref = prog.register_new_func();
        prog
    }

    pub fn get_func_mut(&mut self, func_ref: FuncRef) -> &mut Func {
        &mut self.funcs[func_ref.0]
    }

    pub fn get_func(&self, idx: FuncRef) -> &Func {
        &self.funcs[idx.0]
    }

    pub fn get_main_func_ref(&self) -> FuncRef {
        self.main_func_ref
    }

    pub fn register_new_func(&mut self) -> FuncRef {
        let idx = FuncRef(self.funcs.len());
        let func = Func::new(idx);
        self.funcs.push(func);
        idx
    }
}
