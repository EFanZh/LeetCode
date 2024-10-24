pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_max_game(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let n = nums.len().trailing_zeros();

        for i in 0..n {
            let small_skip = 1 << i;
            let big_skip = small_skip << 2;
            let mut j = small_skip;

            while j < nums.len() {
                nums[j - small_skip] = nums[j - small_skip].min(nums[j]);

                j += big_skip;
            }

            j = small_skip * 3;

            while j < nums.len() {
                nums[j - small_skip] = nums[j - small_skip].max(nums[j]);

                j += big_skip;
            }
        }

        nums[0]
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_max_game(nums: Vec<i32>) -> i32 {
        Self::min_max_game(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
