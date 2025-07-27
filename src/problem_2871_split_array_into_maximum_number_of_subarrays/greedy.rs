pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_subarrays(nums: Vec<i32>) -> i32 {
        let mut and = -1;
        let mut zeros = 0;

        for num in nums {
            and &= num;

            if and == 0 {
                and = -1;
                zeros += 1;
            }
        }

        u32::max(zeros, 1) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_subarrays(nums: Vec<i32>) -> i32 {
        Self::max_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
