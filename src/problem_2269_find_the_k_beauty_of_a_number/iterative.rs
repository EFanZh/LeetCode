pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU32;

impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let num = num as u32;
        let mut k = k as u32;
        let mut x = num;
        let mut base = 1;
        let mut window = 0;

        loop {
            window += base * (x % 10);
            x /= 10;
            k -= 1;

            if k == 0 {
                break;
            }

            base *= 10;
        }

        let mut result = 0;

        loop {
            if NonZeroU32::new(window).is_some_and(|window| num % window == 0) {
                result += 1;
            }

            if x == 0 {
                break;
            }

            window = window / 10 + base * (x % 10);
            x /= 10;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn divisor_substrings(num: i32, k: i32) -> i32 {
        Self::divisor_substrings(num, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
