pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn sort(lhs: &mut (u16, u8), rhs: &mut (u16, u8)) -> bool {
        let needs_swap = rhs.0 > lhs.0;

        if needs_swap {
            mem::swap(lhs, rhs);
        }

        needs_swap
    }

    pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        let mut primary = (a as _, b'a');
        let mut secondary = (b as _, b'b');
        let mut tertiary = (c as _, b'c');

        Self::sort(&mut primary, &mut secondary);

        if Self::sort(&mut secondary, &mut tertiary) {
            Self::sort(&mut primary, &mut secondary);
        }

        primary.0 = primary.0.min((secondary.0 + tertiary.0 + 1) * 2);

        let mut result =
            String::with_capacity(usize::from(primary.0) + usize::from(secondary.0) + usize::from(tertiary.0));

        let mut prev_1 = 0;
        let mut prev_2 = 0;

        loop {
            if prev_1 == prev_2 && primary.1 == prev_2 {
                result.push(char::from(secondary.1));

                secondary.0 -= 1;
                prev_1 = prev_2;
                prev_2 = secondary.1;

                Self::sort(&mut secondary, &mut tertiary);
            } else {
                result.push(char::from(primary.1));

                primary.0 -= 1;
                prev_1 = prev_2;
                prev_2 = primary.1;

                if Self::sort(&mut primary, &mut secondary) {
                    Self::sort(&mut secondary, &mut tertiary);
                } else if primary.0 == 0 {
                    break;
                }
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
        Self::longest_diverse_string(a, b, c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
