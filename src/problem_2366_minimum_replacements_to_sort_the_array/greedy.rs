pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_replacement(nums: Vec<i32>) -> i64 {
        let mut result = 0;
        let mut right = u32::MAX;

        for &num in nums.iter().rev() {
            let num = num as u32;

            if right < num {
                let quotient = num / right;
                let remainder = num % right;

                if remainder == 0 {
                    result += u64::from(quotient - 1);
                } else {
                    result += u64::from(quotient);
                    right = num / (quotient + 1);
                }
            } else {
                right = num;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_replacement(nums: Vec<i32>) -> i64 {
        Self::minimum_replacement(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
