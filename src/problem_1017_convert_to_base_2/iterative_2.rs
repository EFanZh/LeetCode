pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::str;

impl Solution {
    pub fn base_neg2(n: i32) -> String {
        const MAX_DIGITS: usize = 31;

        let mut n = n;
        let mut buffer = [0; MAX_DIGITS];
        let mut i = MAX_DIGITS;

        while n != 0 {
            i -= 1;
            buffer[i] = b'0' + ((n & 1) as u8);
            n = n.div_euclid(-2);
        }

        let result = &buffer[i..];

        if result.is_empty() {
            "0"
        } else {
            str::from_utf8(result).unwrap()
        }
        .to_string()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn base_neg2(n: i32) -> String {
        Self::base_neg2(n)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
