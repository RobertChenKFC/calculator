use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::expr::ValType;

pub const NUM_DIGITS: usize = 16;

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
pub const SEG_DECIMAL: u8 = 3;
pub const SEG_MINUS: u8 = SEG_MID;

const fn get_bitmap<const N: usize>(bit_indices: [u8; N]) -> ValType {
    let mut bitmap = 0;
    let mut i = 0;
    while i < N {
        bitmap |= 1 << bit_indices[i];
        i += 1;
    }
    bitmap as ValType
}

pub const RADIX: usize = 16;
pub const DIGITS: [ValType; RADIX] = [
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

    pub fn to_inline_string(&self) -> String {
        let mut s = String::new();
        for mut value in self.digits {
            let has_decimal = (value >> SEG_DECIMAL) & 1 == 1;
            if has_decimal {
                // Clear out the SEG_DECIMAL bit.
                value ^= 1 << SEG_DECIMAL;
            }
            if value == 1 << SEG_MINUS {
                s.push('-');
                continue;
            }
            if value != 0 {
                let mut found_digit = false;
                for digit in 0..RADIX {
                    if DIGITS[digit] as u8 == value {
                        s.push_str(&format!("{:x}", digit));
                        found_digit = true;
                        break;
                    }
                }
                assert!(found_digit);
            }
            if has_decimal {
                s.push('.');
            }
        }
        s
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

#[cfg(test)]
mod tests {
    use std::array::IntoIter;

    use crate::seven_segment;

    use super::*;

    fn all_digits_in_radix() -> SevenSegment {
        let mut seven_segment = SevenSegment::new();
        for i in 0..RADIX {
            seven_segment.set_value(
                i,
                if i < 10 {
                    SevenSegment::with_decimal(DIGITS[i])
                } else {
                    DIGITS[i]
                } as u8,
            );
        }
        seven_segment
    }

    fn neg_num() -> SevenSegment {
        let mut seven_segment = SevenSegment::new();
        seven_segment.set_value(0, 1 << SEG_MINUS);
        for (i, digit) in (0..10).rev().enumerate() {
            seven_segment.set_value(i + 1, DIGITS[digit] as u8);
        }
        seven_segment
    }

    #[test]
    fn test_display() {
        let seven_segment = all_digits_in_radix();
        assert_eq!(
            seven_segment.to_string(),
            "\
┌─┐   ╷ ╶─┐ ╶─┐ ╷ ╷ ┌─╴ ┌─╴ ╶─┐ ┌─┐ ┌─┐ ┌─┐ ╷   ┌─╴   ╷ ┌─╴ ┌─╴ 
│ │   │ ┌─┘ ╶─┤ └─┤ └─┐ ├─┐   │ ├─┤ └─┤ ├─┤ ├─┐ │   ┌─┤ ├─╴ ├─╴ 
└─┘.  ╵.└─╴.╶─┘.  ╵.╶─┘.└─┘.  ╵.└─┘.╶─┘.╵ ╵ └─┘ └─╴ └─┘ └─╴ ╵   "
        );
        let seven_segment = neg_num();
        // Unlike the multiline string above, Rust has some "fancy" feature that
        // ignores the whitespace following the backslash and newline so that
        // you can format your multiline string in a way you like without
        // affecting the contents of the string. However, in this case, we
        // **do** want the whitespaces on the next line, so we cannot use the
        // backslash. Therefore, we don't use the backslash here, so we have to
        // remove the newly included newline at the start.
        assert_eq!(
            seven_segment.to_string(),
            "
    ┌─┐ ┌─┐ ╶─┐ ┌─╴ ┌─╴ ╷ ╷ ╶─┐ ╶─┐   ╷ ┌─┐                     
╶─╴ └─┤ ├─┤   │ ├─┐ └─┐ └─┤ ╶─┤ ┌─┘   │ │ │                     
    ╶─┘ └─┘   ╵ └─┘ ╶─┘   ╵ ╶─┘ └─╴   ╵ └─┘                     "
                .trim_start_matches('\n')
        );
    }

    #[test]
    fn test_inline_string() {
        let seven_segment = all_digits_in_radix();
        assert_eq!(
            seven_segment.to_inline_string(),
            "0.1.2.3.4.5.6.7.8.9.abcdef"
        );
        let seven_segment = neg_num();
        assert_eq!(seven_segment.to_inline_string(), "-9876543210");
    }
}
