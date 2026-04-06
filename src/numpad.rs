use std::collections::VecDeque;

use console::Term;

use crate::expr::ValType;

pub trait Numpad {
    fn new() -> Self;

    fn get_input(&mut self) -> ValType;
}

// Keys 0-9 correspond to key code 0-9 directly, and thus are omitted here. The
// remaining keys start at key code 10.
pub const KEY_ADD: ValType = 10;
pub const KEY_SUB: ValType = 11;
pub const KEY_MUL: ValType = 12;
pub const KEY_DIV: ValType = 13;
pub const KEY_DOT: ValType = 14;
pub const KEY_CLR: ValType = 15;
pub const KEY_ENTER: ValType = 16;
pub const NO_KEY: ValType = 17;

trait KeyboardNumpad: Numpad {
    fn get_key(&mut self) -> char;

    fn get_input(&mut self) -> ValType {
        loop {
            let c = self.get_key();
            match c {
                '0'..='9' => break c.to_digit(10).unwrap() as ValType,
                '+' => break KEY_ADD,
                '-' => break KEY_SUB,
                '*' => break KEY_MUL,
                '/' => break KEY_DIV,
                '=' => break KEY_ENTER,
                'c' => break KEY_CLR,
                _ => println!("Illegal numpad key: {}", c),
            }
        }
    }
}

pub struct TermNumpad;

impl Numpad for TermNumpad {
    fn new() -> TermNumpad {
        TermNumpad
    }

    fn get_input(&mut self) -> ValType {
        KeyboardNumpad::get_input(self)
    }
}

impl KeyboardNumpad for TermNumpad {
    fn get_key(&mut self) -> char {
        let term = Term::stdout();
        term.read_char().unwrap()
    }
}

pub struct MockNumpad {
    queue: VecDeque<char>,
}

impl MockNumpad {
    pub fn add_input(&mut self, input: char) {
        self.queue.push_back(input)
    }
}

impl Numpad for MockNumpad {
    fn new() -> MockNumpad {
        MockNumpad {
            queue: VecDeque::new(),
        }
    }

    fn get_input(&mut self) -> ValType {
        KeyboardNumpad::get_input(self)
    }
}

impl KeyboardNumpad for MockNumpad {
    fn get_key(&mut self) -> char {
        self.queue.pop_front().unwrap()
    }
}
