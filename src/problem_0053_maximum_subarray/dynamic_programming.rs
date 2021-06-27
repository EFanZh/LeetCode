pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_sum = i32::MIN;
        let mut prev_max_sum = 0;

        for num in nums {
            prev_max_sum = if prev_max_sum <= 0 { num } else { prev_max_sum + num };
            max_sum = max_sum.max(prev_max_sum);
        }

        max_sum
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::max_sub_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
