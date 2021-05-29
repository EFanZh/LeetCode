pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as u32;
        let mut left = 0;
        let mut right = f64::from(c).sqrt() as u32;
        let mut current = right * right;

        while left <= right {
            match current.cmp(&c) {
                Ordering::Less => {
                    current += left * 2 + 1;
                    left += 1;
                }
                Ordering::Equal => return true,
                Ordering::Greater => {
                    right -= 1;
                    current -= right * 2 + 1;
                }
            }
        }

        false
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn judge_square_sum(c: i32) -> bool {
        Self::judge_square_sum(c)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
