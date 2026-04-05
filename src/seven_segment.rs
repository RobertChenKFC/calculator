use std::fmt::{Display, Error, Formatter};

use crate::expr::ValType;

const NUM_DIGITS: usize = 16;

// The pinout of a 7-segment display with decimal point comes
// from this website:
// https://microcontrollerslab.com/wp-content/uploads/2020/01/7-Segment-display-Pin-Configuration.png
// For reference, it looks like this:
//   --5--
//  |     |
//  6     4
//  |     |
//   --7--
//  |     |
//  0     2
//  |     |
//   --1--   3
const SEG_TOP: u8 = 5;
const SEG_TOP_LEFT: u8 = 6;
const SEG_TOP_RIGHT: u8 = 4;
const SEG_MID: u8 = 7;
const SEG_BOTTOM_LEFT: u8 = 0;
const SEG_BOTTOM_RIGHT: u8 = 2;
const SEG_BOTTOM: u8 = 1;
const SEG_DECIMAL: u8 = 3;

const fn get_bitmap<const N: usize>(bit_indices: [u8; N]) -> ValType {
    let mut bitmap = 0;
    let mut i = 0;
    while i < N {
        bitmap |= 1 << bit_indices[i];
        i += 1;
    }
    bitmap as ValType
}

pub const DIGITS: [ValType; 16] = [
    get_bitmap([
        SEG_TOP,
        SEG_TOP_LEFT,
        SEG_BOTTOM_LEFT,
        SEG_BOTTOM,
        SEG_BOTTOM_RIGHT,
        SEG_TOP_RIGHT,
    ]),
    get_bitmap([SEG_TOP_RIGHT, SEG_BOTTOM_RIGHT]),
    get_bitmap([SEG_TOP, SEG_TOP_RIGHT, SEG_MID, SEG_BOTTOM_LEFT, SEG_BOTTOM]),
    get_bitmap([
        SEG_TOP,
        SEG_TOP_RIGHT,
        SEG_MID,
        SEG_BOTTOM_RIGHT,
        SEG_BOTTOM,
    ]),
    get_bitmap([SEG_TOP_LEFT, SEG_MID, SEG_TOP_RIGHT, SEG_BOTTOM_RIGHT]),
    get_bitmap([SEG_TOP, SEG_TOP_LEFT, SEG_MID, SEG_BOTTOM_RIGHT, SEG_BOTTOM]),
    get_bitmap([
        SEG_TOP,
        SEG_TOP_LEFT,
        SEG_MID,
        SEG_BOTTOM_RIGHT,
        SEG_BOTTOM,
        SEG_BOTTOM_LEFT,
    ]),
    get_bitmap([SEG_TOP, SEG_TOP_RIGHT, SEG_BOTTOM_RIGHT]),
    get_bitmap([
        SEG_TOP,
        SEG_TOP_RIGHT,
        SEG_MID,
        SEG_TOP_LEFT,
        SEG_BOTTOM_LEFT,
        SEG_BOTTOM,
        SEG_BOTTOM_RIGHT,
    ]),
    get_bitmap([
        SEG_TOP,
        SEG_TOP_RIGHT,
        SEG_MID,
        SEG_TOP_LEFT,
        SEG_BOTTOM,
        SEG_BOTTOM_RIGHT,
    ]),
    get_bitmap([
        SEG_TOP,
        SEG_TOP_LEFT,
        SEG_TOP_RIGHT,
        SEG_MID,
        SEG_BOTTOM_LEFT,
        SEG_BOTTOM_RIGHT,
    ]),
    get_bitmap([
        SEG_TOP_LEFT,
        SEG_MID,
        SEG_BOTTOM_RIGHT,
        SEG_BOTTOM,
        SEG_BOTTOM_LEFT,
    ]),
    get_bitmap([SEG_TOP, SEG_TOP_LEFT, SEG_BOTTOM_LEFT, SEG_BOTTOM]),
    get_bitmap([
        SEG_TOP_RIGHT,
        SEG_MID,
        SEG_BOTTOM_RIGHT,
        SEG_BOTTOM,
        SEG_BOTTOM_LEFT,
    ]),
    get_bitmap([SEG_TOP_LEFT, SEG_BOTTOM_LEFT, SEG_TOP, SEG_MID, SEG_BOTTOM]),
    get_bitmap([SEG_TOP_LEFT, SEG_BOTTOM_LEFT, SEG_TOP, SEG_MID]),
];

pub struct SevenSegment {
    digits: [u8; NUM_DIGITS],
}

impl SevenSegment {
    pub fn new() -> SevenSegment {
        SevenSegment {
            digits: [0; NUM_DIGITS],
        }
    }

    pub fn set_value(&mut self, index: usize, value: u8) {
        self.digits[index] = value;
    }

    pub fn with_decimal(value: ValType) -> ValType {
        value | (1 << SEG_DECIMAL)
    }

    fn get_display_char(
        value: u8,
        bit_indices: &[u8],
        display_chars: &[char],
    ) -> char {
        let mut bitmap = 0;
        for (i, index) in bit_indices.iter().rev().enumerate() {
            bitmap |= (((value >> index) & 1) << i) as usize;
        }
        display_chars[bitmap]
    }
}

impl Display for SevenSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        //   0 1 2 3
        // 0 ┌ ─ ┐
        //
        // 1 ├ ─ ┤
        //
        // 2 └ ─ ┘ .

        // Row 0
        for digit in self.digits {
            let top_left = SevenSegment::get_display_char(
                digit,
                &[SEG_TOP, SEG_TOP_LEFT],
                &[' ', '╷', '╶', '┌'],
            );
            let mid =
                SevenSegment::get_display_char(digit, &[SEG_TOP], &[' ', '─']);
            let top_right = SevenSegment::get_display_char(
                digit,
                &[SEG_TOP, SEG_TOP_RIGHT],
                &[' ', '╷', '╴', '┐'],
            );
            write!(f, "{}{}{} ", top_left, mid, top_right)?;
        }
        write!(f, "\n")?;

        // Row 1
        for digit in self.digits {
            let left = SevenSegment::get_display_char(
                digit,
                &[SEG_TOP_LEFT, SEG_BOTTOM_LEFT, SEG_MID],
                &[' ', '╶', '╷', '┌', '╵', '└', '│', '├'],
            );
            let mid =
                SevenSegment::get_display_char(digit, &[SEG_MID], &[' ', '─']);
            let right = SevenSegment::get_display_char(
                digit,
                &[SEG_TOP_RIGHT, SEG_BOTTOM_RIGHT, SEG_MID],
                &[' ', '╴', '╷', '┐', '╵', '┘', '│', '┤'],
            );
            write!(f, "{}{}{} ", left, mid, right)?;
        }
        write!(f, "\n")?;

        // Row 2
        for digit in self.digits {
            let bottom_left = SevenSegment::get_display_char(
                digit,
                &[SEG_BOTTOM, SEG_BOTTOM_LEFT],
                &[' ', '╵', '╶', '└'],
            );
            let mid = SevenSegment::get_display_char(
                digit,
                &[SEG_BOTTOM],
                &[' ', '─'],
            );
            let bottom_right = SevenSegment::get_display_char(
                digit,
                &[SEG_BOTTOM, SEG_BOTTOM_RIGHT],
                &[' ', '╵', '╴', '┘'],
            );
            let decimal = SevenSegment::get_display_char(
                digit,
                &[SEG_DECIMAL],
                &[' ', '.'],
            );
            write!(f, "{}{}{}{}", bottom_left, mid, bottom_right, decimal)?;
        }

        Ok(())
    }
}
