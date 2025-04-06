pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as u32;

        for left in 0..=(c / 2).isqrt() {
            let right_squared = c - left * left;
            let right = right_squared.isqrt();

            if right * right == right_squared {
                return true;
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
