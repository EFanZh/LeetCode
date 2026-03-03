pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut prev = i32::MIN;
        let mut result = 0;

        for &num in &nums {
            let value = (prev + 1).max(num - k).min(num + k);

            result += i32::from(value != prev);

            prev = value;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        Self::max_distinct_elements(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
