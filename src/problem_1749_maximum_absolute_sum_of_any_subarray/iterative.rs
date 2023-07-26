pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_sum = 0;
        let mut max_sum = 0;
        let mut max_abs_sum = 0;

        for num in nums {
            sum += num;

            max_abs_sum = max_abs_sum.max(if num < 0 {
                min_sum = min_sum.min(sum);

                max_sum - sum
            } else {
                max_sum = max_sum.max(sum);

                sum - min_sum
            });
        }

        max_abs_sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        Self::max_absolute_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
