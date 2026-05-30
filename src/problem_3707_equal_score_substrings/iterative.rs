pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn score_balance(s: String) -> bool {
        let mut right = s.bytes().map(u32::from).sum::<u32>() - u32::from(b'a' - 1) * s.len() as u32;
        let mut left = 0;

        for c in s.bytes() {
            let c = u32::from(c - (b'a' - 1));

            left += c;
            right -= c;

            match left.cmp(&right) {
                Ordering::Less => {}
                Ordering::Equal => return true,
                Ordering::Greater => break,
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn score_balance(s: String) -> bool {
        Self::score_balance(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
